import * as React from "react";
import {Account} from './domain/account';
import {ActionCreators} from './flux/action_creators';
import {AccountLine} from './account_line';
import {AccountPage} from './flux/state/account_page';

interface AccountPageComponentProps {
    actions: ActionCreators,
    page: AccountPage
}

export function AccountPageComponent(props: AccountPageComponentProps) {
    if (props.page.accounts === null) {
        return <div>Loading your accounts...</div>;
    }
    let accounts = props.page.accounts.map((account: Account, i: number) => 
        <AccountLine key={i} actions={props.actions} account={account} isOpen={props.page.openAccount === String(account.id)} openOptions={props.page.openOptions}/>);
    return <div>
        {accounts}
        </div>;
}