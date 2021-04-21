export interface ErrorModel {
    status: boolean;
    message: string;
}

export interface RegisterResponseModel {
    status: boolean;
    message: string;
}

export interface AccessToken {
    token: string;
    deadline: string;
}
