import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';

interface LoginPageProps {
    page: LoginPage,
    dispatch: (action: Action) => void
}

export function SignupPage(props: LoginPageProps) {
    let actions = new ActionCreators();
    return <div className="signup-form">
        <form className="form-signin">
            <div className="form-row">
              <div>Cambio allows you to buy, sell and trade Ethereum with ease</div>
            </div>
            <div className="form-row">
              <label className="sr-only">Email address</label>
              <input type="email" id="inputEmail" className="form-control" value={props.page.emailAddress} placeholder="Email address" 
                onChange={(e: any) => props.dispatch(actions.setEmailAddress(e.target.value as string))}>
              </input>
            </div>
            <div className="form-row">
              <label className="sr-only">Password</label>
              <input type="password" id="inputPassword" className="form-control" value={props.page.password} placeholder="Password" 
                onChange={(e: any) => props.dispatch(actions.setPassword(e.target.value as string))}>
              </input>
            </div>
            <LoginButton signupMode={true}></LoginButton>
            <LoginMessage></LoginMessage>
            <LoginOptions signupMode={false}></LoginOptions>
          </form>
      </div>;
}

interface LoginButtonProps {
    signupMode: boolean
}

function LoginButton(props: LoginButtonProps) {
    if (props.signupMode) {
        return <div className="form-row">
              <button className="btn btn-lg btn-primary btn-block" type="submit">Sign in</button>
            </div>;
    } else {
        return <div className="form-row">
              <button className="btn btn-lg btn-primary btn-block" type="submit">Create account</button>
            </div>;
    }
}

function LoginMessage() {
    var loginFailed = false;
    if (loginFailed) {
        return <div className="form-row error-text">
            <em>Logging in failed. Check your email address and password and try again.</em>
        </div>;
    }
    return null;
}

function LoginOptions(props: LoginButtonProps) {
    if (props.signupMode) {
        return <div className="form-row">
          <a href="javascript: void">I already have an account.</a> 
        </div>;
    } else {
        return <div className="form-row">
          Don't have an account? <a href="javascript: void">Create one.</a>.
        </div>;
    }
}
