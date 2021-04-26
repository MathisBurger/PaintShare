import React from "react";
import DesignWrapper from "../components/designWrapper";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import style from "../styles/profile.module.css";
import {useParams} from "react-router-dom";
import {faUpload} from "@fortawesome/free-solid-svg-icons";

// This interface defines the types of the given
// url params
interface ParamTypes {
    name: string
}

export default function Profile() {

    const { name } = useParams<ParamTypes>();

    return (
        <>
            <DesignWrapper>
                <div className={style.profileOuterBox}>
                    <div className={style.profileBox}>
                        <img src={"https://upload.wikimedia.org/wikipedia/commons/c/cd/Black_from_a_camera.jpg"} alt={"profile picture"}/>
                        <div className={style.rightBox}>
                            {profileBoxOptions(name)}
                            {statsBox(name)}
                        </div>
                    </div>
                    {handleUploadSection(name)}
                </div>
            </DesignWrapper>
        </>
    );
}

// This function handles whether the profile page is the
// profile page of the logged in user, or another
// profile page of another user
function profileBoxOptions(name: any): any {

    if (name === undefined) {
        return (
            <>
                <div className={style.upperBox}>
                    <h2>username</h2>
                    <button>edit profile</button>
                </div>
            </>
        );
    } else {
        return <h1>username</h1>;
    }
}

// This function handles the style of the
// stats box of the profile page
function statsBox(name: any): any {

    if (name === undefined) {
       return (
           <div className={style.lowerBox}>
               <div className={style.content}>
                   <h1>12</h1>
                   <p>posts</p>
               </div>
               <div className={style.content}>
                   <h1>10M</h1>
                   <p>subs</p>
               </div>
               <div className={style.content}>
                   <h1>120</h1>
                   <p>follows</p>
               </div>
           </div>
       );
    } else {
        return (
            <div className={`${style.lowerBox} ${style.unowned}`}>
                <div className={style.content}>
                    <h1>12</h1>
                    <p>posts</p>
                </div>
                <div className={style.content}>
                    <h1>10M</h1>
                    <p>subs</p>
                </div>
                <div className={style.content}>
                    <h1>120</h1>
                    <p>follows</p>
                </div>
            </div>
        );
    }
}

function handleUploadSection(name: any): any {

    if (name === undefined) {

        return (
            <>
                <div className={style.hr} />
                <button className={style.uploadButton} ><FontAwesomeIcon icon={faUpload} className={style.margin}/>Upload</button>
                <div className={style.hr} />
            </>
        );
    } else {

        return <></>;
    }
}
