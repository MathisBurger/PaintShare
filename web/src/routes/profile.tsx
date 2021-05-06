import React, {useState} from "react";
import DesignWrapper from "../components/designWrapper";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import style from "../styles/profile.module.css";
import {useHistory, useParams} from "react-router-dom";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import {UserAPI} from "../services/api/user";
import {getTempURL} from "../services/utils";
import UploadContainer from "../components/uploadContainer";
import ReactDOM from "react-dom";

// This interface defines the types of the given
// url params
interface ParamTypes {
    name: string
}

export default function Profile() {

    const history = useHistory();

    const { name } = useParams<ParamTypes>();

    const [url, changeURL] = useState("");
    const [showUpload, changeShowUpload] = useState(false);

    lazyLoader();

    return (
        <>
            {showUpload ? <UploadContainer /> : null}
            <DesignWrapper>
                <div className={style.profileOuterBox}>
                    <div className={style.profileBox}>
                        <img src={url} alt={"profile picture"}/>
                        <div className={style.rightBox}>
                            {profileBoxOptions(name)}
                            {statsBox(name)}
                        </div>
                    </div>
                    {handleUploadSection(name, changeShowUpload)}
                </div>
            </DesignWrapper>
        </>
    );

    async function lazyLoader() {
        if (url === "") {
            let pic_data : any = await new UserAPI().getProfilePictureURL(name);
            if (pic_data == null) {
                history.push("/login");
            } else {
                changeURL(getTempURL(pic_data, pic_data.data));
            }
        }
    }
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

function handleUploadSection(name: any, changePopupState: any): any {

    if (name === undefined) {

        return (
            <>
                <div className={style.hr} />
                <button className={style.uploadButton} onClick={e => changePopupState(true)}
                ><FontAwesomeIcon icon={faUpload} className={style.margin}/>new post</button>
                <div className={style.hr} />
            </>
        );
    } else {

        return <></>;
    }
}

