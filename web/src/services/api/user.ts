import {RestImplementation} from "./implementation";
import {BaseResponse} from "../../typings/api/BaseResponse";
import {GetPostsResponse} from "../../typings/api/GetPostsResponse";
import {User} from "../../typings/api/models/user";
import {Props} from "react";
import {CheckFollowStateResponse} from "../../typings/api/CheckFollowState";
import {SearchUserResponse} from "../../typings/api/SearchUserResponse";

const PREFIX = process.env.NODE_ENV === "development" ? "http://127.0.0.1:8080/api": "/api";

// The UserAPI handles all requests focusing the user API
export class UserAPI {

    // This function tries to login the user with the given credentials
    // It returns the status as promise of a boolean value
    async login(username: string, password: string): Promise<boolean> {
        let resp = await fetch(`${PREFIX}/auth/login`, {
            method: "POST",
            mode: process.env.NODE_ENV === "development" ? "cors" : "same-origin",
            headers: {
                "Content-Type": "application/json"
            },
            credentials: 'include',
            body: JSON.stringify({username, password})
        });
        console.log(resp.status);
        console.log(resp.statusText);
        console.log(resp.headers);
       return resp.status === 200;
    }

    // This function tries to fetch the profile picture of
    // the given username. If the username matches undefined,
    // the api returns the profile picture of the session owner
    async getProfilePictureURL(name: any): Promise<any> {
        return await RestImplementation.get("/user-api/get_profile_picture" + (name === undefined ? "": "?username=" + name), false, true);
    }

    // This function tries to fetch the information of
    // the user identified by the given user_id. It
    // only returns general information about the user.
    async getUserInformation(user_id: number = 0, username: string = ""): Promise<User> {
        if (user_id !== 0) return await RestImplementation.get<User>(`/user-api/get_user_information?user_id=${user_id}`, true, false);
        if (username !== "") return await RestImplementation.get<User>(`/user-api/get_user_information?username=${username}`, true, false);
        else throw "wrong implementation of getUserInformation: neither user_id or username given, but one of them is required";
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
        const path = "/user-api/get_posts" + (name === undefined ? "" : ("?user=" + name.name));
        return await RestImplementation.get<GetPostsResponse | BaseResponse>(path);
    }

    // This function queries the subscription
    // state of the given user_id. It checks if
    // the user who owns the session follows the user
    // with the given user_id
    async checkFollowState(user_id: number): Promise<CheckFollowStateResponse | BaseResponse> {
        return await RestImplementation.get<CheckFollowStateResponse | BaseResponse>("/user-api/follow_state?user_id=" + user_id);
    }

    // This function requests to follow
    // the user, identified by the given user_id.
    // The status is being returned as a instance of BaseResponse
    async followUser(user_id: number): Promise<BaseResponse> {
        return await RestImplementation.post<BaseResponse>("/user-api/follow_user", {user_id: user_id});
    }

    // This function requests to unfollow
    // the user, identified by the given user_id.
    // The status is being returned as a instance of BaseResponse
    async unfollowUser(user_id: number): Promise<BaseResponse> {
        return await RestImplementation.post<BaseResponse>("/user-api/unfollow_user", {user_id: user_id});
    }

    // This function fetches all user accounts with a displayname
    // similar to the search value and returns them as the
    // SearchUserResponse type
    async searchUser(search: string): Promise<SearchUserResponse> {
        return await RestImplementation.get<SearchUserResponse>("/user-api/search_user?searchword=" + search);
    }
}
