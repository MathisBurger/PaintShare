import EventEmitter from "events";
import { AccessToken, ErrorModel } from "./models";


const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";


export class RestImplementation {

    public static readonly events = new EventEmitter();

    private static accessToken: AccessToken;

    private static getAccessToken(): Promise<AccessToken> {
        return this.get<AccessToken>("/auth/accesstoken?username=" + localStorage.getItem("username"));
    }

    public static get<T>(path: string, emitError: boolean = true, blob: boolean = false): Promise<T> {
        return this.req<T>("GET", path, undefined, undefined, emitError, 0, blob);
    }

    public static post<T>(
        path: string,
        body?: any,
        emitError: boolean = true,
        contentType: string | undefined = "application/json"
    ): Promise<T> {
        return this.req<T>("POST", path, body, contentType, emitError);
    }


    private static async req<T>(
        method: string,
        path: string,
        body?: any,
        contentType: string | undefined = "application/json",
        emitError: boolean = true,
        counter: number = 0,
        blob: boolean = false
    ): Promise<T> {

        if (this.accessToken && new Date(+this.accessToken.deadline * 1000) < new Date()) {
            try {
                if (counter > 3) {
                    return "expired" as any;
                }
                this.accessToken = await this.getAccessToken();
                let newCounter = counter + 1;
                return this.req(method, path, body, contentType, emitError, newCounter, blob);
            } catch (e) {
                if (emitError) this.events.emit("error", e);
                throw e;
            }
        }

        let reqBody = undefined;
        if (body) {
            if (typeof body !== "string" && contentType === "application/json") {
                reqBody = JSON.stringify(body);
            } else {
                reqBody = body;
            }
        }

        const headers: { [key: string]: string } = {};
        if (contentType !== "multipart/form-data") {
            headers["content-type"] = contentType;
        }

        if (this.accessToken) {
            headers["authorization"] = "accessToken " + this.accessToken.token;
        }

        const res = await window.fetch(`${PREFIX}${path}`, {
            method,
            headers,
            body: reqBody,
            credentials: "include",
        });

        if (res.status === 401) {
            try {
                if (!blob) {
                    const resBody = (await res.json()) as ErrorModel;
                    if (resBody.message === "invalid access token") {
                        this.accessToken = await this.getAccessToken();
                        if (counter < 3) {
                            return this.req(method, path, body, contentType, emitError, counter + 1, blob);
                        } else {
                            return resBody as any;
                        }
                    }
                } else {
                    this.accessToken = await this.getAccessToken();
                    if (counter < 3) {
                        return this.req(method, path, body, contentType, emitError, counter + 1, blob);
                    } else {
                        return null as any;
                    }
                }
            } catch {}
            if (emitError) this.events.emit("authentication-error", res);
        }

        if (res.status === 204 || res.headers.get("content-length") === "0") {
            return {} as T;
        }
        if (blob) {
            return await res.blob() as any;
        } else {
            return res.json();
        }
    }
}
