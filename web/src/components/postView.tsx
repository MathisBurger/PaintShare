import React from "react";

import style from "../styles/postView.module.css";
import {PostAPI} from "../services/api/posts";
import {UserAPI} from "../services/api/user";
import {getTempURL} from "../services/utils";
import {PostViewProps} from "../typings/components/postView";


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
        const userInfo = await new UserAPI().getUserInformation(postInfo.post.owner_id);
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
        console.log(this.state);
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
                        </div>
                        </>
                    )
                    }
                    <div className={style.postViewRightContainer}>
                        <div className={style.postViewHeader}>
                        </div>
                    </div>
                </div>
            </>
        );
    }
}
