import {Post} from "./models/post";
import {Comment} from "./models/comment";
import {Like} from "./models/like";

export interface GetPostData {
    post: Post;
    comments: Comment[];
    likes: Like[];
}
