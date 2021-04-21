import React from "react";

import baseStyle from "../styles/base.module.css";
import formStyle from "../styles/forms.module.css";

export default function Register() {

    return (
      <>
          <div className={baseStyle.fullScreen} style={{
              background: `url(${process.env.PUBLIC_URL + '/login-bg.jpg'}) no-repeat center center fixed`
          }}>
              <div className={`${formStyle.loginContainer} ${baseStyle.center} `}>
                  <h1>Register</h1>
                  <input type={"text"} placeholder={"username"}/>
                  <input type={"text"} placeholder={"email"} />
                  <input type={"password"} placeholder={"password"} />
                  <input type={"password"} placeholder={"retype password"} />
                  <button>Register</button>
                  <a href={"/login"}>Login</a>
              </div>
          </div>
      </>
    );
}
