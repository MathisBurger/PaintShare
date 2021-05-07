import React from "react";

import style from "../styles/postGroup.module.css";

export default function PostGroup(props: any) {

    return (
        <>
            <div className={style.postGroupContainer}>
                {props.children}
            </div>
        </>
    );
}
