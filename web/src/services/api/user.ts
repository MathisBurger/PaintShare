import {RestImplementation} from "./implementation";

const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";

// The UserAPI handles all requests focusing the user API
export class UserAPI {

    // This function tries to login the user with the given credentials
    // It returns the status as promise of a boolean value
    async login(username: string, password: string): Promise<boolean> {
        let resp = await fetch(`${PREFIX}/auth/login`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            credentials: 'include',
            body: JSON.stringify({username, password})
        });
       return resp.status === 200;
    }

    async getProfilePictureURL(name: any) {
        return await RestImplementation.get("/user-api/get_profile_picture", true);
    }
}
