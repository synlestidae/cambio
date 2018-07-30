import * as React from "react";
import {LoginPage} from './flux/state/login_page';
import {AccountPage} from './flux/state/account_page';
import {AppState} from './flux/app_state';
import {SignupPage} from './signup_page';
import {AccountPageComponent} from './account_page_component';
import {MyAccountPageComponent} from './my_account_page_component';
import {BoardPageComponent} from './board_page_component';
import {Action} from './flux/action'
import {ActionCreators} from './flux/action_creators'
import {BoardPage} from './flux/state/board_page';
import {MyAccount} from './flux/state/my_account';

export interface ContentComponentProps {
    state: AppState,
    actions: ActionCreators
}

export function ContentComponent(props: ContentComponentProps) {
    return <div id="main-content">
      <div className="page-container">
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
    if (props.state.page instanceof AccountPage) {
        let accountPage: AccountPage = props.state.page as AccountPage;
        return <div>
            <AccountPageComponent page={accountPage} actions={props.actions}>
            </AccountPageComponent>
        </div>;
    }
    if (props.state.page instanceof BoardPage) {
        let accountPage: BoardPage = props.state.page as BoardPage;
        return <div>
            <BoardPageComponent page={accountPage} actions={props.actions}>
            </BoardPageComponent>
        </div>;
    }
    if (props.state.page instanceof MyAccount) {
        let myAccountPage: MyAccount = props.state.page as MyAccount;
        return <div>
            <MyAccountPageComponent page={myAccountPage} actions={props.actions}>
            </MyAccountPageComponent>
        </div>;
    }
    return null;
}
