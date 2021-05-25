import {User} from "./models/user";

export interface SearchUserResponse {
    status: boolean;
    message: string;
    user: User[];
}
