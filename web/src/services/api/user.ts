import {RestImplementation} from "./implementation";
import {BaseResponse} from "../../typings/api/BaseResponse";
import {GetPostsResponse} from "../../typings/api/GetPostsResponse";

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

    // This function tries to fetch the profile picture of
    // the given username. If the username matches undefined,
    // the api returns the profile picture of the session owner
    async getProfilePictureURL(name: any) {
        return await RestImplementation.get("/user-api/get_profile_picture" + (name === undefined ? "": "?username=" + name), false, true);
    }

    // This function tries to fetch the upload endpoint
    // of the user API. It saves the multipart/form-data
    // image to the server and saves it as a post
    async uploadPost(image: any, comment: string, tags: string): Promise<BaseResponse> {
        return await RestImplementation.post<BaseResponse>("/user-api/upload_post?comment=" + comment + "&tags=" + tags, image, true, "multipart/form-data");
    }

    // This endpoint fetches the basic metadata of every
    // post published by the user, who owns the given
    // profile.
    async getAllPosts(name: any): Promise<GetPostsResponse | BaseResponse> {
        const path = "/user-api/get_posts" + (name === undefined ? ("&user=" + name): "");
        return await RestImplementation.get<GetPostsResponse | BaseResponse>(path);
    }
}
