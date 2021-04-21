import React from "react";

import baseStyle from "../styles/base.module.css";
import formStyle from "../styles/forms.module.css";

export default function Login() {

    return (
        <>
            <div className={baseStyle.fullScreen} style={{
                background: `url(${process.env.PUBLIC_URL + '/login-bg.jpg'}) no-repeat center center fixed`
            }}>
                <div className={`${formStyle.loginContainer} ${baseStyle.center} `}>
                    <h1>Login</h1>
                    <input type={"text"} placeholder={"username"}/>
                    <input type={"password"} placeholder={"password"} />
                    <button>Login</button>
                    <a href={"/register"}>Register</a>
                </div>
            </div>
        </>
    );
}
