import {ActionCreators} from './flux/action_creators';
import * as React from 'react';

interface AccountBalanceProps {
    balance: string;
    availableBalance: string;
}

export function AccountBalance(props: AccountBalanceProps) {
    return <div>
        <div>Balance: ${props.balance}</div>
        <div>Available: ${props.availableBalance}</div>
    </div>;
}
