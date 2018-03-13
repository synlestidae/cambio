import * as React from "react";

export class SignupPage extends React.Component<any, any> {
    constuctor(props: any) {
    }

    componentDidMount() {
        this.setState({
            emailAddress: '',
            password: '',
            signupMode: false
        });
    }

    render() {
        if (this.state === null) {
            return null;
        }
        return (
     <div className="signup-form">
          <form className="form-signin">
            <div className="form-row">
              <div>Cambio allows you to buy, sell and trade Ethereum with ease</div>
            </div>
            <div className="form-row">
              <label className="sr-only">Email address</label>
              <input type="email" id="inputEmail" className="form-control" value={this.state.emailAddress} placeholder="Email address">
              </input>
            </div>
            <div className="form-row">
              <label className="sr-only">Password</label>
              <input type="password" id="inputPassword" className="form-control" value={this.state.password} placeholder="Password">
              </input>
            </div>
            <LoginButton signupMode={this.state.signupMode}></LoginButton>
            <LoginMessage></LoginMessage>
            <LoginOptions signupMode={this.state.signupMode}></LoginOptions>
          </form>
      </div>);
    }
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
