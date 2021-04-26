import React, {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faHome, faSearch, faUser} from "@fortawesome/free-solid-svg-icons";

import style from "../styles/sidebar.module.css";

export default function Sidebar() {

    return (
        <>
            <div className={style.sidebar}>
                <div className={style.sidebarHeading}>
                    PaintShare
                </div>
                <ul>
                    <li>
                        <FontAwesomeIcon icon={faHome} />
                        <div className={style.name}>Home</div>
                    </li>
                    <li>
                        <FontAwesomeIcon icon={faSearch} />
                        <div className={style.name}>Explore</div>
                    </li>
                    <li>
                        <FontAwesomeIcon icon={faUser} />
                        <div className={style.name}>Profile</div>
                    </li>
                </ul>
            </div>
        </>
    );
}
