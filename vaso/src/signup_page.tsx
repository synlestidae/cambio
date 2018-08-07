import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';
import {buildSignupForm} from './build_signup_form';
import {SignupState} from './flux/state/signup_state';

interface LoginPageProps {
    page: LoginPage,
    actions: ActionCreators
}

export function SignupPage(props: LoginPageProps): JSX.Element {
    return <div className="signup-form">
        <PageForm {...props}></PageForm>;
    </div>
}

function PageForm(props: LoginPageProps): JSX.Element {
    if (props.page.isSignup) {
        return null;
    }
    return <LoginForm {...props}></LoginForm>;
}

function LoginForm(props: LoginPageProps): JSX.Element {
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
