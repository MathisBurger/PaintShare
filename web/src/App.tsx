import React from 'react';
import './App.css';
// @ts-ignore
import {BrowserRouter as Router, Route, Switch} from "react-router-dom";

import Login from "./routes/Login";
import Register from "./routes/Register";
import Dashboard from "./routes/dashboard";
import Profile from "./routes/profile";

function App() {
  return (
    <Router>
      <Switch>
        <Route path={"/login"} component={Login} />
        <Route path={"/register"} component={Register} />
        <Route path={"/dashboard"} component={Dashboard} />
        <Route path={"/profile"} component={Profile} />
          <Route path={"/user/:name"} component={Profile} />
      </Switch>
    </Router>
  );
}

export default App;
