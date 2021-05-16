import {GetPostData} from "../api/GetPostData";
import {User} from "../api/models/user";

export interface PostViewProps {
    loading: boolean;
    imageURL?: string;
    postInfo?: GetPostData;
    userInfo?: User;
    profilePictureURL?: string;
    commentField: string;
}
