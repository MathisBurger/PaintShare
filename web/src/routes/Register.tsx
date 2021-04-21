import React, { useState } from "react";
// @ts-ignore
import { useHistory } from "react-router-dom";
import { TokenlessAPI } from "../services/api/tokenless";

import baseStyle from "../styles/base.module.css";
import formStyle from "../styles/forms.module.css";

export default function Register() {

    const history = useHistory();

    const [username, changeUsername] = useState("");
    const [email, changeEmail] = useState("");
    const [pwd, changePwd] = useState("");
    const [retypePwd, changeRetypePwd] = useState("");

    let handleRegister = () =>  {

        if (pwd === retypePwd) {
            if (username !== "" && email !== "" && pwd !== "") {
                new TokenlessAPI().register(username, email, pwd).then(res => {
                    if (res.status) {
                        history.push("/login");
                    } else {
                        alert(res.message);
                    }
                })
            } else {
                alert("Please fill out every field");
            }
        } else {
            alert("Passwords are not equal");
        }
    }

    return (
      <>
          <div className={baseStyle.fullScreen} style={{
              background: `url(${process.env.PUBLIC_URL + '/login-bg.jpg'}) no-repeat center center fixed`
          }}>
              <div className={`${formStyle.loginContainer} ${baseStyle.center} `}>
                  <h1>Register</h1>
                  <input type={"text"} placeholder={"username"} onChange={e => changeUsername(e.target.value)} />
                  <input type={"email"} placeholder={"email"} onChange={e => changeEmail(e.target.value)} required={true} />
                  <input type={"password"} placeholder={"password"} onChange={e => changePwd(e.target.value)} />
                  <input type={"password"} placeholder={"retype password"} onChange={e => changeRetypePwd(e.target.value)} />
                  <button onClick={handleRegister}>Register</button>
                  <a href={"/login"}>Login</a>
              </div>
          </div>
      </>
    );
}
