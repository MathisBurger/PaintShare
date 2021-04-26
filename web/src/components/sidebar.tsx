import React, {useState} from "react";

import style from "../styles/sidebar.module.css";

export default function Sidebar() {

    return (
        <>
            <div className={style.sidebar}>
                <div className={style.sidebarHeading}>
                    PaintShare
                </div>
                <ul>
                    <li>Home</li>
                    <li>Explore</li>
                </ul>
            </div>
        </>
    );
}
