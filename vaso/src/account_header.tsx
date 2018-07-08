import {ActionCreators} from './flux/action_creators';
import * as React from 'react';

interface AccountHeaderProps {
    actions: ActionCreators,
    accountId: string
}

export function AccountHeader(props: AccountHeaderProps): JSX.Element{
    return <div className="side-by-side">
        <div>
          Cash Wallet (NZD) 
        </div>
        <div className="account-credit-operations">
            <button className="btn non-touching-button" onClick={() => props.actions.creditAccount(props.accountId)}>
              Credit
            </button>
            <button className="btn non-touching-button" onClick={() => props.actions.cashOutAccount(props.accountId)}>
              Cash out
            </button>
        </div>
    </div>;
}
