import React from "react";

import style from "../styles/post.module.css";
import {useAsync} from "react-async";
import {PostAPI} from "../services/api/posts";

export default function PostComponent(postID: any) {

    const {data} = useAsync({promiseFn: new PostAPI().getPostImage, post_id: postID})
    return (
        <>
            <div className={style.post}>
                <img src={data} alt={"post image"} />
            </div>
        </>
    );


}
