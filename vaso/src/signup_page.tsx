import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';
import {SignupForm} from './signup_form';

interface LoginPageProps {
    page: LoginPage,
    actions: ActionCreators
}

export function SignupPage(props: LoginPageProps) {
    if (props.page.isSignup) {
        let signup = SignupForm(Object.assign({}, props.page.signupState, {actions: props.actions}));
        return <div className="signup-form">
            <form className="form-signin">
                <div className="form-row">
                  <div>Enter your login details.</div>
                </div>
                {signup}
                <SignupButton {...props.page.signupState} actions={props.actions}>
                </SignupButton>    
            </form>
        </div>;
    }
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
      </div>
}

interface LoginButtonProps {
    isSignup: boolean,
    emailAddress: string,
    password: string,
    actions: ActionCreators
}

function SignupButton(props: SignupState & {actions: ActionCreators}) {
    let next = ''
    let prev = '';
    let nextPage = '';

    if (props.form_state === 'LoginInfo') {
        next = 'Add personal details';
        prev = 'I have an account.';
    }

    if (props.form_state === 'PersonalInfo') {
        next = 'Identify yourself (optional)';
        prev = 'Edit personal details';
    }

    if (props.form_state === 'ConfirmEmail') {
        next = 'Prove your ID';
        prev = 'Back to personal details';
    }

    if (props.form_state === 'Identification') {
        next = 'Finish';
        prev = 'Back to personal details';
    }

    return <div className="form-row">
        <a href="javascript: void" onClick={(e: any) => {e.preventDefault();  props.actions.prevSignupForm()}}>{prev}</a> 
        <button onClick={(e: any) => {e.preventDefault(); props.actions.nextSignupForm()}} className="btn btn-primary btn-block width-initial" type="submit">
          {next}
        </button>
    </div>;
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
          <span>Don't have an account?</span>
          <span>
            <a href="javascript: void" onClick={() => props.actions.signupMode()}>Create one</a>
          </span>
        </div>;
    }
}
