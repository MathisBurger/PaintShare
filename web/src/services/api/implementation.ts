import EventEmitter from "events";
import { AccessToken, ErrorModel } from "./models";


const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";

export class RestImplementation {

    public static readonly events = new EventEmitter();

    private static accessToken: AccessToken;

    private static getAccessToken(): Promise<AccessToken> {
        return this.get<AccessToken>("auth/accesstoken");
    }

    private static get<T>(path: string, emitError: boolean = true): Promise<T> {
        return this.req<T>("GET", path, undefined, undefined, emitError);
    }

    private static post<T>(
        path: string,
        body?: any,
        emitError: boolean = true
    ): Promise<T> {
        return this.req<T>("POST", path, body, undefined, emitError);
    }


    private static async req<T>(
        method: string,
        path: string,
        body?: any,
        contentType: string | undefined = "application/json",
        emitError: boolean = true
    ): Promise<T> {
        if (this.accessToken && new Date(this.accessToken.deadline) < new Date()) {
            console.log("Access token expired");
            try {
                this.accessToken = await this.getAccessToken();
                return this.req(method, path, body, contentType, emitError);
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

        const res = await window.fetch(`${PREFIX}/${path}`, {
            method,
            headers,
            body: reqBody,
            credentials: "include",
        });

        if (res.status === 401) {
            try {
                const resBody = (await res.json()) as ErrorModel;
                if (resBody.message === "invalid access token") {
                    this.accessToken = await this.getAccessToken();
                    return this.req(method, path, body, contentType, emitError);
                }
            } catch {}
            if (emitError) this.events.emit("authentication-error", res);
            throw new Error("auth error");
        }

        if (!res.ok) {
            if (emitError) this.events.emit("error", res);
            throw new Error(res.statusText);
        }

        if (res.status === 204 || res.headers.get("content-length") === "0") {
            return {} as T;
        }

        return res.json();
    }
}
