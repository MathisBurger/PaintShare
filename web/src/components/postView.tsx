import React from "react";

import style from "../styles/postView.module.css";
import {PostAPI} from "../services/api/posts";
import {UserAPI} from "../services/api/user";
import {getTempURL} from "../services/utils";
import {PostViewProps} from "../typings/components/postView";
import {User} from "../typings/api/models/user";
import {Like} from "../typings/api/models/like";
import {GetPostData} from "../typings/api/GetPostData";


export default class PostView extends React.Component<any, any>{

    state: PostViewProps = {
        loading: true
    };
    wrapperRef: React.RefObject<any>;


    constructor(props: any) {
        super(props);
        this.wrapperRef = React.createRef();
        this.handleClickOutside = this.handleClickOutside.bind(this);
    }

    // This function loads all required data
    // from the backend, that is required
    // to display the postView
    async componentDidMount() {

        // Event to register clicks outside the view
        document.addEventListener('mousedown', this.handleClickOutside);

        const url = await new PostAPI().getPostImage(this.props.postID);
        const postInfo = await new PostAPI().getPostData(this.props.postID);
        const userInfo: User = await new UserAPI().getUserInformation(postInfo.post.owner_id);
        const profilePicture = await new UserAPI().getProfilePictureURL(userInfo.displayname);
        const profilePictureURL = getTempURL(profilePicture, profilePicture.type)
        this.setState({imageURL: url, loading: false, postInfo, userInfo, profilePictureURL});
    }


    componentWillUnmount() {
        // Event to register clicks outside the view
        document.removeEventListener('mousedown', this.handleClickOutside);
    }

    // Closes the postView of user clicks outside of the postView
    handleClickOutside(event: any) {
        if (this.wrapperRef && !this.wrapperRef.current.contains(event.target)) {
            this.props.closer();
        }
    }

    // This function is being executed if the user
    // likes a post. It sends the like request to
    // the server and appends a new virtual like to
    // the postInfo state to instantly update the like
    // state in the frontend
    async likePost(): Promise<void> {
        let resp = await new PostAPI().likePost(this.state.postInfo?.post.id as number);
        if (resp.status) {
            let likes = this.state.postInfo?.likes;
            likes?.push({
                like_id: 0,
                post_id: this.state.postInfo?.post.id,
                owner: localStorage.getItem("username"),
                timestamp: 0
            } as Like);
            this.setState({postInfo: {
                post: this.state.postInfo?.post,
                comments: this.state.postInfo?.comments,
                likes: likes
                } as GetPostData
            });
        } else {
            alert(resp.message);
        }
    }

    render() {
        return (
            <>
                <div className={style.postViewContainer} ref={this.wrapperRef}>
                    {this.state.loading ? (
                        <div>loading...</div>
                    ): (
                        <>
                        <div className={style.postViewLeftContainer} style={{backgroundImage: `url(${this.state.imageURL})`}} />
                        <div className={style.postViewRightContainer}>
                            <div className={style.postViewHeader}>
                                <img src={this.state.profilePictureURL}  alt={"profile picture"} />
                                <a
                                    href={`${window.location.protocol}//${window.location.host}/profile/${this.state.userInfo?.displayname}`}
                                >
                                    {this.state.userInfo?.displayname}
                                </a>
                            </div>
                            <div className={style.postViewCommentSection}>
                                {this.state.postInfo?.comments.map((comment, i) => {
                                    return (
                                        <div className={style.comment}>
                                            <a
                                                href={`${window.location.protocol}//${window.location.host}/profile/${comment.owner}`}
                                            >
                                                {comment.owner}
                                            </a>
                                            <p>{comment.message}</p>
                                        </div>
                                    );
                                })}
                            </div>
                            <input type={"text"} className={style.postViewWriteComment} placeholder={"write comment..."}/>
                            <div className={style.postViewFooter}>
                                <div className={`${style.heart} ${
                                    checkUserLikedPost(this.state.postInfo?.likes as Like[]) ?
                                        null : style.unliked
                                }`} onClick={e => this.likePost()}/>
                                <div className={style.content}>
                                    <h1>{this.state.postInfo?.post.likes}</h1>
                                    <p>Likes</p>
                                </div>
                                <div className={style.content}>
                                    <h1>{this.state.postInfo?.post.comments}</h1>
                                    <p>Comments</p>
                                </div>
                            </div>
                        </div>
                        </>
                    )
                    }
                </div>
            </>
        );
    }
}

// This function checks if the active
// user liked the shown post, by iterating
// through all the likes and checking if
// the username is owner of one
function checkUserLikedPost(likes: Like[]): boolean {
    let username = localStorage.getItem("username");
    for (let i=0; i<likes.length; i++) {
        if (likes[0].owner === username) {
            return true;
        }
    }
    return false;
}
