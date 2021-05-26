import React from "react";
import DesignWrapper from "../components/designWrapper";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPoop} from "@fortawesome/free-solid-svg-icons";

import style from "../styles/explore.module.css";
import {UserAPI} from "../services/api/user";
import {ExploreProps, ExploreState} from "../typings/routes/Explore";
import {User} from "../typings/api/models/user";
import {getTempURL} from "../services/utils";

export default class ExploreRoute extends React.Component<ExploreProps, ExploreState> {

    state: ExploreState = {
        userProfilePictureURLs: [],
        userFound: false
    };

    // This function fetches all matching users
    // and their profile pictures. After fetching this
    // data it updates the state. But only if the search field
    // is not empty
    async searchUser(e: React.ChangeEvent<HTMLInputElement>) {
        if (e.target.value !== "") {
            let results = await new UserAPI().searchUser(e.target.value);
            if (results.status) {
                let urls = [];
                for (const user of results.user) {
                    let blob = await new UserAPI().getProfilePictureURL(user.displayname);
                    urls.push(getTempURL(blob, blob.data))
                }
                this.setState({searchResults: results.user, userFound: true, userProfilePictureURLs: urls});
            } else {
                this.setState({userFound: false});
            }
        } else {
            this.setState({userFound: false});
        }
    }


    render() {
        return (
            <>
                <DesignWrapper>
                    <div>
                        <input
                            className={style.searchBox}
                            placeholder={"search username"}
                            onChange={e => this.searchUser(e)}
                        />
                        {this.state.userFound ? (
                            <div className={style.contentView}>
                                {this.state.searchResults?.map((u: User, index: number) => {
                                    return (
                                        <div
                                            className={style.card}
                                            onClick={() => {
                                                window.location.href = `${window.location.protocol}//${window.location.host}/user/${u.displayname}`
                                            }}
                                        >
                                            <img src={this.state.userProfilePictureURLs[index]}/>
                                            <div className={style.rightBox}>
                                                <h2>{u.displayname}</h2>
                                                <div className={style.bottomBox}>
                                                    <div className={style.statusContent}>
                                                        <h1>{u.num_follower}</h1>
                                                        <p>Follower</p>
                                                    </div>
                                                    <div className={style.statusContent}>
                                                        <h1>{u.num_subscriptions}</h1>
                                                        <p>Subscriptions</p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    );
                                })}
                            </div>
                        ) : (
                            <div className={style.notFoundDialog}>
                                <div className={style.centeredPoop}>
                                    <FontAwesomeIcon icon={faPoop} size={"4x"} style={{color: "#9191a2"}}/>
                                </div>
                                <h1>No mathing user found</h1>
                            </div>
                        )}
                    </div>
                </DesignWrapper>
            </>
        );
    }
}
