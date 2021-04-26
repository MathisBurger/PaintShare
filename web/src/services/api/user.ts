
const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";

// The UserAPI handles all requests focusing the user API
export class UserAPI {

    // This function trys to login the user with the given credentials
    // It returns the status as promise of a boolean value
    async login(username: string, password: string): Promise<boolean> {

        let resp = await fetch(`${PREFIX}/auth/login`, {
            method: "POST",
            mode: "cors",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: username,
                password: password
            })
        });

       return resp.status === 200;
    }
}
