import {Post} from "../api/models/post";

export function emptyPostArray(): Post[] {
    const post = {
        owner_id: 0,
        id: 0,
        comment: "",
        created_at: 0
    } as Post;
    return [post];
}
