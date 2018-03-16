import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';

interface LoginPageProps {
    page: LoginPage,
    actions: ActionCreators
}

export function SignupPage(props: LoginPageProps) {
    return <div className="signup-form">
        <form className="form-signin">
            <div className="form-row">
              <div>Cambio allows you to buy, sell and trade Ethereum with ease</div>
            </div>
            <div className="form-row">
              <label className="sr-only">Email address</label>
              <input type="email" id="inputEmail" className="form-control" value={props.page.emailAddress} placeholder="Email address" 
                onChange={(e: any) => props.actions.setEmailAddress(e.target.value as string)}>
              </input>
            </div>
            <div className="form-row">
              <label className="sr-only">Password</label>
              <input type="password" id="inputPassword" className="form-control" value={props.page.password} placeholder="Password" 
                onChange={(e: any) => props.actions.setPassword(e.target.value as string)}>
              </input>
            </div>
            <div className="form-row">
              <LoginButton emailAddress={props.page.emailAddress} password={props.page.password} isSignup={props.page.isSignup} actions={props.actions}>
              </LoginButton>
            </div>
            <LoginMessage page={props.page}></LoginMessage>
            <LoginOptions isSignup={props.page.isSignup} actions={props.actions}></LoginOptions>
          </form>
      </div>;
}

interface LoginButtonProps {
    isSignup: boolean,
    emailAddress: string,
    password: string,
    actions: ActionCreators
}

function LoginButton(props: LoginButtonProps) {
    let text: string;
    const callback = (e: any) => {
        e.preventDefault();
        if (props.isSignup) {
            props.actions.submitSignup(props.emailAddress, props.password);
        } else {
            props.actions.submitLogin(props.emailAddress, props.password);
        }
    };
    if (props.isSignup) {
        text = 'Create account';
    } else {
        text = 'Sign in';
    }
    return <button className="btn btn-lg btn-primary btn-block" type="submit" onClick={callback}>
        {text}
        </button>;
}

interface LoginMessageProps {
    page: LoginPage
}

function LoginMessage(props: LoginMessageProps) {
    let loginFailed = props.page.loadingState.name === 'Error';
    if (loginFailed) {
        return <div className="form-row error-text">
            <em>Logging in failed. Check your email address and password and try again.</em>
        </div>;
    }
    return null;
}


interface LoginOptionsProps {
    actions: ActionCreators,
    isSignup: boolean
}

function LoginOptions(props: LoginOptionsProps) {
    if (props.isSignup) {
        return <div className="form-row">
          <a href="javascript: void" onClick={() => props.actions.loginMode()}>I already have an account.</a> 
        </div>;
    } else {
        return <div className="form-row">
          Don't have an account? <a href="javascript: void" onClick={() => props.actions.signupMode()}>Create one.</a>.
        </div>;
    }
}
