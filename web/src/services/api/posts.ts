import {RestImplementation} from "./implementation";
import {getTempURL} from "../utils";
import {GetPostData} from "../../typings/api/GetPostData";
import {BaseResponse} from "../../typings/api/BaseResponse";

export class PostAPI {

    // This function fetches the get_post_image endpoint
    // and downloads the post image. After that it
    // generates a temporary link based on the blob
    // data of the response.
    // It is designed for the use with a react hook
    async getPostImageWithHook(post_id: any): Promise<string> {
        let resp = await RestImplementation.get(`/post-api/get_post_image?post_id=${post_id.post_id.postID}`, true, true) as any;
        return getTempURL(resp, resp.type);
    }

    // This function fetches the get_post_image endpoint
    // and downloads the post image. After that it
    // generates a temporary link based on the blob
    // data of the response.
    async getPostImage(post_id: any): Promise<string> {
        let resp = await RestImplementation.get(`/post-api/get_post_image?post_id=${post_id}`, true, true) as any;
        return getTempURL(resp, resp.type);
    }

    // This function fetches the get_post_data endpoint
    // and returns the meta data of a specific post.
    // It includes comments and likes
    async getPostData(post_id: any): Promise<GetPostData> {
        return await RestImplementation.get<GetPostData>("/post-api/get_post_data?post_id=" + post_id, true, false);
    }

    // This function sends a request to the server
    // to like the post with the given post_id
    // as the user, who owns the active session
    async likePost(post_id: number): Promise<BaseResponse> {
        return await RestImplementation.post<BaseResponse>("/post-api/like_post", {post_id: post_id});
    }
}
