import React, {useState} from "react";
import DesignWrapper from "../components/designWrapper";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import style from "../styles/profile.module.css";
import {useHistory, useParams} from "react-router-dom";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import {UserAPI} from "../services/api/user";
import {getTempURL} from "../services/utils";
import UploadContainer from "../components/uploadContainer";
import PostGroup from "../components/postGroup";
import {useAsync} from "react-async";
import {Post} from "../typings/api/models/post";
import {GetPostsResponse} from "../typings/api/GetPostsResponse";
import PostComponent from "../components/post";
import PostView from "../components/postView";

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

    const { data } = useAsync({promiseFn: new UserAPI().getAllPosts, name: undefined});

    var image_data: Post[] = [];
    if (data?.status) {
        image_data = (data as GetPostsResponse).posts;
    }

    lazyLoader();

    return (
        <>
            <PostView postID={18} />
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
                    <PostGroup>
                        {handlePosts(image_data)}
                    </PostGroup>
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

// This function handles the upload section
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

function handlePosts(posts: Post[]) {
    let arr = [];
    for (let i=0; i<posts.length; i++) {
        arr.push(<PostComponent postID={posts[i].id} />);
    }
    return arr;
}
