import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';
import {buildForm, SignupForm, FormElem} from './signup_form';

interface LoginPageProps {
    page: LoginPage,
    actions: ActionCreators
}

export function SignupPage(props: LoginPageProps) {
    if (props.page.isSignup) {
        if (props.page.signupState.form_state === 'ConfirmEmail') {
            return <ConfirmEmail {...props}></ConfirmEmail>;
        }
        let formProps = Object.assign({}, props.page.signupState, {actions: props.actions});
        let formElems = buildForm(formProps);
        let signup = SignupForm(formElems, props.actions, props.page.signupState);
        return <div className="signup-form">
            <form className="form-signin" onClick={(e: any) => e.preventDefault()}>
                <div className="form-row">
                  <div>Enter your login details.</div>
                </div>
                {signup}
                <SignupButton {...props.page.signupState} actions={props.actions} formElems={formElems}>
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

function formElemValid(b: boolean, e: FormElem) {
    return b && e.validate(e.value) == null;
}

function SignupButton(props: SignupState & {actions: ActionCreators} & {formElems: FormElem[]}) {
    let next = ''
    let prev = 'Back';
    let nextPage = '';
    let allValid = props.formElems.reduce(formElemValid, true)

    if (props.form_state === 'LoginInfo') {
        next = 'Add personal details';
    }

    if (props.form_state === 'PersonalInfo') {
        next = 'Identify yourself (optional)';
    }

    if (props.form_state === 'ConfirmEmail') {
        next = 'Prove your ID';
    }

    if (props.form_state === 'Identification') {
        next = 'Finish';
    }

    const nextFn = function(e: any) { 
        if (props.form_state === 'LoginInfo') {
            props.actions.sendRegistration(props.loginInfo, props.info);
        }
        props.actions.nextSignupForm();
    };

    const prevFn = function(e: any) { 
        e.preventDefault();
        props.actions.prevSignupForm();
    };

    //let validationMessage = props.formElems.reduce((elem: FormElem, msg: string|null) => msg || elem.validate(elem.value), null);

    return <div className="form-row">
        <button onClick={nextFn} className="btn btn-primary btn-block width-initial" disabled={!allValid}>
          {next}
        </button>
        <a href="javascript: void" onClick={prevFn}>{prev}</a> 
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

function ConfirmEmail(props: LoginPageProps): JSX.Element {
    let actions = props.actions; 
    let state = props.page.signupState;

    return (<div className="form-signin"> 
        <div className="form-row">
          <div>Enter the 5-digit confirmation code that was emailed to {props.page.signupState.loginInfo.email_address}.</div>
        </div>
        <div className="form-row">
          <input 
              type="text" 
              maxLength={5} 
              className="pin-input form-control" 
              onChange={(e: any) => actions.setConfirmationCode(e.target.value as string)}
              value={props.page.signupState.confirmationCode} >
          </input>
        </div>
        <div className="form-row side-by-side">
            <button className="btn width-initial" onClick={() => 
                actions.resendEmail(state.loginInfo.email_address, state.registrationInfo.identifierCode)
            }>
                Resend email
            </button>
            <button
                className="btn btn-primary btn-block width-initial" 
                onClick={async function() {
                    await props.actions.confirmRegistration(state);
                    props.actions.submitLogin(state.loginInfo.email_address, state.loginInfo.password);
                }}>
                Confirm email
            </button>
        </div>
        <div className="form-row">
            <a href="javascript: void(0)" onClick={() => actions.prevSignupForm()}>Back</a>
        </div>
    </div>);
}
