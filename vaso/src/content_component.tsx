import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {AppState} from './flux/app_state';
import {SignupPage} from './signup_page';
import {Action} from './flux/action'
import {ActionCreators} from './flux/action_creators'

export interface ContentComponentProps {
    state: AppState,
    actions: ActionCreators
}

export function ContentComponent(props: ContentComponentProps) {
    return <div id="main-content">
      <div className="page-container signup-container">
        <PageComponent state={props.state} actions={props.actions}/>
      </div>
    </div>;
}

function PageComponent(props: ContentComponentProps) {
    if (props.state.page instanceof LoginPage) {
        let signupPage: LoginPage = props.state.page as LoginPage;
        return <div>
            <SignupPage page={signupPage} actions={props.actions}>
            </SignupPage>
        </div>;
    }
    return null;
}
