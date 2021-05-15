import React from "react";

import style from "../styles/post.module.css";
import {useAsync} from "react-async";
import {PostAPI} from "../services/api/posts";

export default function PostComponent(postID: any) {

    const {data} = useAsync({promiseFn: new PostAPI().getPostImageWithHook, post_id: postID})

    const background = {
        backgroundImage: `url(${data})`
    };

    return <div className={style.post} style={background} />;
}
