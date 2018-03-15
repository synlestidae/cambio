import * as React from "react";
import {AccountPage} from './flux/state/account_page';
import {ActionCreators} from './flux/action_creators';

interface AccountPageComponentProps {
    actions: ActionCreators,
    page: AccountPage
}

export function AccountPageComponent(props: AccountPageComponentProps) {
    return <div>Oooooooooh! Accounts</div>
}
