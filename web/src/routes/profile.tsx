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
import {User} from "../typings/api/models/user";

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

    const [showPostView, changeShowPostView] = useState(false);
    const [showedPostID, changeShowedPostID] = useState(0);

    const [userInfo, changeUserInfo] = useState<User>({
        displayname: "",
        num_follower: 0,
        num_subscriptions: 0,
        user_id: 0
    });
    const [userInfoSet, changeUserInfoSet] = useState(false);

    const showPost = (postID: number) => {
        changeShowedPostID(postID);
        changeShowPostView(true);
    }

    const closePostView = () => {
        changeShowPostView(false);
    }

    const { data } = useAsync({promiseFn: new UserAPI().getAllPosts, name: undefined});

    var image_data: Post[] = [];
    if (data?.status) {
        image_data = (data as GetPostsResponse).posts;
    }

    lazyLoader(name);

    return (
        <>
            {showPostView ? <PostView postID={showedPostID} closer={closePostView}/> : null}
            {showUpload ? <UploadContainer /> : null}
            <DesignWrapper>
                <div className={style.profileOuterBox}>
                    <div className={style.profileBox}>
                        <img src={url} alt={"profile picture"}/>
                        <div className={style.rightBox}>
                            {profileBoxOptions(userInfo.displayname, window.location.pathname === "/profile")}
                            <div className={style.lowerBox}>
                                <div className={style.content}>
                                    <h1>{userInfo.num_follower}</h1>
                                    <p>subs</p>
                                </div>
                                <div className={style.content}>
                                    <h1>{userInfo.num_subscriptions}</h1>
                                    <p>follows</p>
                                </div>
                            </div>
                        </div>
                    </div>
                    {handleUploadSection(name, changeShowUpload)}
                    <PostGroup>
                        {image_data.map((post, i) => {
                            return <div onClick={e => showPost(post.id)}><PostComponent postID={post.id} clicker={showPost} /></div>
                        })}
                    </PostGroup>
                </div>
            </DesignWrapper>
        </>
    );

    async function lazyLoader(name: string) {
        if (url === "") {
            let pic_data : any = await new UserAPI().getProfilePictureURL(name);
            if (pic_data == null) {
                history.push("/login");
            } else {
                changeURL(getTempURL(pic_data, pic_data.data));
            }
        }
        if (!userInfoSet) {
            if (name === undefined) {
                let nm = localStorage.getItem("username");
                if (nm === null) {
                    throw "username is not declared in local storage";
                }
                changeUserInfo(await new UserAPI().getUserInformation(0, nm));
            } else {
                changeUserInfo(await new UserAPI().getUserInformation(0, name));
            }
            changeUserInfoSet(true);
        }
    }
}

// This function handles whether the profile page is the
// profile page of the logged in user, or another
// profile page of another user
function profileBoxOptions(name: string, ownProfile: boolean): any {

    if (ownProfile) {
        return (
            <>
                <div className={style.upperBox}>
                    <h2>{name}</h2>
                    <button>edit profile</button>
                </div>
            </>
        );
    } else {
        return <h1>{name}</h1>;
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

