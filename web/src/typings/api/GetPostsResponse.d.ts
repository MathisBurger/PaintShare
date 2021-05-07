import {Post} from "./models/post";

export interface GetPostsResponse {
    status: boolean;
    username: string;
    posts: Post[];
}
