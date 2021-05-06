import React, {useState} from "react";
import style from "../styles/uploadWrapper.module.css";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faTimes} from "@fortawesome/free-solid-svg-icons";
import {UserAPI} from "../services/api/user";

export default function UploadContainer() {

    const [uploadedImage, changeUploadedImage] = useState(false);
    const [imageSRC, changeImageSRC] = useState("");

    const [comment, changeComment] = useState("");
    const [tags, changeTags] = useState("");

    const renderImage = (e: any) => {
        changeImageSRC(URL.createObjectURL(e.target.files[0]));
        changeUploadedImage(true);
    };

    const uploadPost = (comment: string, tags: string) => {
        // @ts-ignore
        let img = (document.getElementById("upload_post_input") as HTMLInputElement).files[0];
      var formData = new FormData();
      formData.append("image", img);
      new UserAPI().uploadPost(formData, comment, tags).then(resp => {
          console.log(resp);
        if (!resp.status) {
            alert("ERROR" +  resp.message);
        } else {
            window.location.reload();
        }
      })
    };

    return (
        <>
            <div className={`${style.uploadContainer}`}>
                <div className={style.headContainer}>
                    <h1>New post</h1>
                    <FontAwesomeIcon icon={faTimes} onClick={e => window.location.reload()} />
                </div>
                <div className={style.bodyContainer}>
                    <div className={style.leftContainer}>
                        <input type={"file"} onChange={e => renderImage(e)} id={"upload_post_input"} />
                        {uploadedImage ? <img src={imageSRC}  alt={""} /> : null}
                    </div>
                    <div className={style.rightContainer}>
                        <input type={"text"} placeholder={"comment"} onChange={e => changeComment(e.target.value)} />
                        <textarea placeholder={"separate tags with a space"} onChange={e => changeTags(e.target.value)} />
                        <button onClick={e => uploadPost(comment, tags)}>Upload</button>
                    </div>
                </div>
            </div>
        </>
    );
}
