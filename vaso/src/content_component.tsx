import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {AppState} from './flux/app_state';
import {SignupPage} from './signup_page';
import {Action} from './flux/action'

export interface ContentComponentProps {
    state: AppState,
    dispatch: (state: Action) => void
}

export function ContentComponent(props: ContentComponentProps) {
    return <div id="main-content">
      <div className="page-container signup-container">
        <PageComponent state={props.state} dispatch={props.dispatch}/>
      </div>
    </div>;
}

function PageComponent(props: ContentComponentProps) {
    if (props.state.page instanceof LoginPage) {
        let signupPage: LoginPage = props.state.page as LoginPage;
        return <div>
            <SignupPage page={signupPage} dispatch={props.dispatch}></SignupPage>
        </div>;
    }
    return null;
}
