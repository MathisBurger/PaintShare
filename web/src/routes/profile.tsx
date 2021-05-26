import React, {useState} from "react";
import DesignWrapper from "../components/designWrapper";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import style from "../styles/profile.module.css";
import {useHistory, useParams} from "react-router-dom";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import {faCamera} from "@fortawesome/free-solid-svg-icons";
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
import {CheckFollowStateResponse} from "../typings/api/CheckFollowState";
import {UserInfo, userInfo} from "os";

// This interface defines the types of the given
// url params
interface ParamTypes {
    name: string
}

export default function Profile() {

    const history = useHistory();

    const { name } = useParams<ParamTypes>();

    const [url, changeURL] = useState<string>("");
    const [showUpload, changeShowUpload] = useState<boolean>(false);

    const [showPostView, changeShowPostView] = useState<boolean>(false);
    const [showedPostID, changeShowedPostID] = useState<number>(0);

    const [userInfo, changeUserInfo] = useState<User>({
        displayname: "",
        num_follower: 0,
        num_subscriptions: 0,
        user_id: 0
    });
    const [userInfoSet, changeUserInfoSet] = useState<boolean>(false);
    const [subscriptionState, changeSubscriptionState] = useState<boolean>(false);

    const showPost = (postID: number) => {
        changeShowedPostID(postID);
        changeShowPostView(true);
    }

    const closePostView = () => {
        changeShowPostView(false);
    }

    const { data } = useAsync({promiseFn: new UserAPI().getAllPosts, name: name});

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
                            {profileBoxOptions(
                                userInfo.displayname,
                                window.location.pathname === "/profile",
                                subscriptionState,
                                userInfo
                            )}
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
                    {image_data.length === 0 ? (
                        <div className={style.noImagesBox}>
                            <FontAwesomeIcon icon={faCamera} size={"3x"} />
                            <h2>No uploaded images</h2>
                        </div>
                    ) : (
                        <PostGroup>
                            {image_data.map((post, i) => {
                                return <div onClick={e => showPost(post.id)}><PostComponent postID={post.id} clicker={showPost} /></div>
                            })}
                        </PostGroup>
                    )}
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
        if (name !== undefined) {
            let resp = (await new UserAPI().checkFollowState(userInfo.user_id)) as CheckFollowStateResponse;
            changeSubscriptionState(resp.check_status);
        }
    }
}

// This function handles whether the profile page is the
// profile page of the logged in user, or another
// profile page of another user
function profileBoxOptions(
    name: string,
    ownProfile: boolean,
    subscriptionState: boolean,
    userInfo: User
): any {

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
        return (
            <>
                <div className={style.upperBox}>
                    <h2>{name}</h2>
                    <button
                        style={{
                        backgroundColor: subscriptionState ? "" : "#0083d8",
                        color: subscriptionState ? "black" : "white",
                        marginLeft: "0.5em",
                        border: "none"
                    }}
                        onClick={() => {
                            if (subscriptionState) {
                                new UserAPI().unfollowUser(userInfo.user_id).then(() => {
                                    window.location.reload();
                                });
                            } else {
                                new UserAPI().followUser(userInfo.user_id).then(() => {
                                    window.location.reload();
                                });
                            }
                        }}
                    >
                        {subscriptionState ? "Unsubscribe" : "Subscribe"}
                    </button>
                </div>
            </>
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

