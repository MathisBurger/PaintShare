import React from "react";

import style from "../styles/postView.module.css";
import {useAsync} from "react-async";
import {PostAPI} from "../services/api/posts";

export default function PostView(postID: any) {

    const url = useAsync({promiseFn: new PostAPI().getPostImage, post_id: postID}).data;
    const background = {
        backgroundImage: `url(${url})`
    };

    return (
        <>
            <div className={style.postViewContainer}>
                <div className={style.postViewLeftContainer}  style={background} />
            </div>
        </>
    );
}
