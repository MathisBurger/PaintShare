import React from "react";

import style from "../styles/postView.module.css";
import {PostAPI} from "../services/api/posts";
import {UserAPI} from "../services/api/user";
import {getTempURL} from "../services/utils";
import {PostViewProps} from "../typings/components/postView";
import {User} from "../typings/api/models/user";


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

    async componentDidMount() {
        document.addEventListener('mousedown', this.handleClickOutside);
        const url = await new PostAPI().getPostImage(this.props.postID);
        const postInfo = await new PostAPI().getPostData(this.props.postID);
        const userInfo: User = await new UserAPI().getUserInformation(postInfo.post.owner_id);
        const profilePicture = await new UserAPI().getProfilePictureURL(userInfo.displayname);
        const profilePictureURL = getTempURL(profilePicture, profilePicture.type)
        this.setState({imageURL: url, loading: false, postInfo, userInfo, profilePictureURL});
    }

    componentWillUnmount() {
        document.removeEventListener('mousedown', this.handleClickOutside);
    }

    handleClickOutside(event: any) {
        if (this.wrapperRef && !this.wrapperRef.current.contains(event.target)) {
            this.props.closer();
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
                                <div className={`${style.heart} ${style.unliked}`} />
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
