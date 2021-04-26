import React, {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faHome, faSearch, faUser} from "@fortawesome/free-solid-svg-icons";

import style from "../styles/sidebar.module.css";
import {useHistory} from "react-router-dom";

export default function Sidebar() {

    const history = useHistory();

    return (
        <>
            <div className={style.sidebar}>
                <div className={style.sidebarHeading}>
                    PaintShare
                </div>
                <ul>
                    <li onClick={e => history.push("/dashboard")} >
                        <FontAwesomeIcon icon={faHome} />
                        <div className={style.name}>Home</div>
                    </li>
                    <li onClick={e => history.push("/explore")} >
                        <FontAwesomeIcon icon={faSearch} />
                        <div className={style.name}>Explore</div>
                    </li>
                    <li onClick={e => history.push("/profile")} >
                        <FontAwesomeIcon icon={faUser} />
                        <div className={style.name}>Profile</div>
                    </li>
                </ul>
            </div>
        </>
    );
}
