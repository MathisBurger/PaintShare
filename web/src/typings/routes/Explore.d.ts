import {User} from "../api/models/user";

export interface ExploreProps {}

export interface ExploreState {
    userFound: boolean;
    searchResults?: User[];
    userProfilePictureURLs: string[];
}
