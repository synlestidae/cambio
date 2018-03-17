import * as React from "react";
import {Account} from './domain/Account';
import {AccountPage} from './flux/state/account_page';
import {ActionCreators} from './flux/action_creators';
import {AccountLine} from './account_line';

interface AccountPageComponentProps {
    actions: ActionCreators,
    page: AccountPage
}

export function AccountPageComponent(props: AccountPageComponentProps) {
    if (props.page.accounts === null) {
        return <div>Loading your accounts...</div>;
    }
    let accounts = props.page.accounts.map((account: Account, i: number) => <AccountLine key={i}/>);
    return <div>
        {accounts}
        </div>;
}
