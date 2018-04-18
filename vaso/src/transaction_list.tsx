import * as React from "react";
import {LoadingState} from './flux/state/loading_state';
import {Transaction} from './domain/transaction';
//import {ActionCreators} from './flux/action_creators';

export interface TransactionListProps {
    //actions: ActionCreat
    loadingState: LoadingState,
    transactions: Array<Transaction>|null;
}

export function TransactionList(props: TransactionListProps) {
    if (props.loadingState.name === 'Loading') {
        return <div className="container">Loading this account's transactions...</div>
    }
    if (props.loadingState.name === 'Error') {
        let msg = props.loadingState.message;
        let text = `Error while loading transactions${msg? ': ' + msg : ''}.`;
        return <div className="container error-text">
            {text}
            </div>
    }
    if (props.transactions === null || props.transactions.length === 0) {
        return <div className="container">No transactions yet!</div>;
    }
    let txRows = getRows(props.transactions);
    return <div className="container">
        <table style={{width: '100%'}}>
          <tr>
            <th>Time</th>
            <th>Type</th>
            <th>Amount</th>
            <th>Message</th>
            <th>Balance</th>
          </tr>
          {txRows}
        </table>
        </div>;
}

function getRows(transactions: Transaction[]) {
    return transactions.map((t: Transaction, i: number) => {
        //let time = Date.parse(t.transaction_time);
        let formattedTime = formatUTC(t.transaction_time);
        return <tr key={i}>
            <td>{formattedTime}</td>
            <td>{t.business_ends}</td>
            <td>{formatCents(t.value)}</td>
            <td>{t.message}</td>
            <td>{formatCents(t.balance)}</td>
        </tr>;
    });
}

function formatCents(cents: number) {
    var dollars = cents / 100;
    return dollars.toFixed(2);
}

function formatUTC(time: string) {
    let date = new Date(Date.parse(time));
    return formatTime(date);
}

function formatTime(time: Date) {
    let date = `${time.getFullYear()}-${time.getMonth() + 1}-${time.getDate()}`;
    let hoursMinutes = `${pad(time.getHours())}:${pad(time.getMinutes())}`;
    return `${date} ${hoursMinutes}`;
}

function pad(thing: any) {
    let str = String(thing);
    if (str.length < 2) {
        return '0' + str;
    }
    return str;
}
