import { RegisterResponseModel } from "./models";

const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";

export class TokenlessAPI {

    async register(username: string, mail: string, pwd: string): Promise<RegisterResponseModel> {

        let resp = await fetch(`${PREFIX}/user/register`, {
            method: "POST",
            mode: "cors",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                displayname: username,
                email: mail,
                password: pwd
            })
        });

        return (await resp.json()) as RegisterResponseModel;
    }
}
