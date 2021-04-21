import React from 'react';
import './App.css';
// @ts-ignore
import {BrowserRouter as Router, Route, Switch} from "react-router-dom";

import Login from "./routes/Login";
import Register from "./routes/Register";

function App() {
  return (
    <Router>
      <Switch>
        <Route path={"/login"} component={Login} />
        <Route path={"/register"} component={Register} />
      </Switch>
    </Router>
  );
}

export default App;
