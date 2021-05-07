import {RestImplementation} from "./implementation";
import {getTempURL} from "../utils";

export class PostAPI {

    // This function fetches the get_post_image endpoint
    // and downloads the post image. After that it
    // generates a temporary link based on the blob
    // data of the response
    async getPostImage(post_id: any): Promise<string> {
        let resp = await RestImplementation.get("/post-api/get_post_image?post_id=" + post_id.post_id.postID, true, true) as any;
        return getTempURL(resp, resp.type);
    }
}
