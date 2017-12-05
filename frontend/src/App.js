import React, { Component } from 'react';
import { withRouter } from 'react-router';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'

import GithubLogin, {GithubOauthWindow} from './github-login-component';
import {CreateAccount} from './create-account';

import logo from './logo.svg';
import './App.css';

class App extends Component {
  render() {
    return (
      <div className="WobscaleAccounts">
        <header className="Wobscale-Login">
          <img src={logo} className="Wobscale-logo" alt="logo" />
        </header>
        <Router>
          <div>
            <Route
              exact
              path="/"
              render={(props) => <MainPage {...props}/>}
              />
            <Route
              path="/github/oauth"
              render={(props) => <GithubOauthWindow {...props}/>}
              />
            <Route
              path="/account/create"
              render={(props) => <CreateAccount {...props}/>}
              />
          </div>
        </Router>
      </div>
    );
  }
}

const MainPage = withRouter((props) => (
  <div>
    <h1 className="App-title">Login</h1>
    <p>
      Login to your Wobscale Account
    </p>
    <GithubLogin {...props}/>

    <h2>Create an account</h2>
    <p> If you don't have an account yet, login to create one </p>
  </div>
));

export default App;