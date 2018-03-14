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
              <LoginButton emailAddress={props.page.emailAddress} password={props.page.password} signupMode={false} actions={props.actions}>
              </LoginButton>
            </div>
            <LoginMessage page={props.page}></LoginMessage>
            <LoginOptions signupMode={false} actions={props.actions}></LoginOptions>
          </form>
      </div>;
}

interface LoginButtonProps {
    signupMode: boolean,
    emailAddress: string,
    password: string,
    actions: ActionCreators
}

function LoginButton(props: LoginButtonProps) {
    let text: string;
    const callback = (e: any) => {
        e.preventDefault();
        if (props.signupMode) {
            throw new Error('Not yet implemented!');
        } else {
            props.actions.submitLogin(props.emailAddress, props.password);
        }
    };
    if (props.signupMode) {
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
    console.log('loading boi', props.page.loadingState);
    if (loginFailed) {
        return <div className="form-row error-text">
            <em>Logging in failed. Check your email address and password and try again.</em>
        </div>;
    }
    return null;
}


interface LoginOptionsProps {
    actions: ActionCreators,
    signupMode: boolean
}

function LoginOptions(props: LoginOptionsProps) {
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
