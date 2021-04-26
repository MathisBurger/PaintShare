import React, {useState} from "react";

import baseStyle from "../styles/base.module.css";
import formStyle from "../styles/forms.module.css";
import {UserAPI} from "../services/api/user";
// @ts-ignore
import {useHistory} from "react-router-dom";

export default function Login() {

    const history = useHistory();

    const [username, changeUsername] = useState("");
    const [pwd, changePwd] = useState("");

    let handleLogin = () => {
        new UserAPI().login(username, pwd).then(res => {
            if (!res) {
               alert("Login falied");
            } else {
                history.push("/dashboard");
            }
        })
    }

    return (
        <>
            <div className={baseStyle.fullScreen} style={{
                background: `url(${process.env.PUBLIC_URL + '/login-bg.jpg'}) no-repeat center center fixed`
            }}>
                <div className={`${formStyle.loginContainer} ${baseStyle.center} `}>
                    <h1>Login</h1>
                    <input type={"text"} placeholder={"username"} onChange={e => changeUsername(e.target.value)} />
                    <input type={"password"} placeholder={"password"} onChange={e => changePwd(e.target.value)} />
                    <button onClick={handleLogin} >Login</button>
                    <a href={"/register"}>Register</a>
                </div>
            </div>
        </>
    );
}
