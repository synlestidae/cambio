import * as React from "react";
import {LoadingState} from './flux/state/loading_state';
import {Transaction} from './domain/transaction';
import {padZeroes} from './pad_zeroes';
import {Table} from './table/table';
import {ReactTableVisitor} from './table/react_table_visitor';
import {FieldColumn} from './table/field_column';
import {DateFieldColumn} from './table/date_field_column';
import {DollarsFieldColumn} from './table/dollars_field_column';
//import {ActionCreators} from './flux/action_creators';

export interface TransactionListProps {
    loadingState: LoadingState,
    transactions: Array<Transaction>|null;
}

export function TransactionList(props: TransactionListProps) {
    if (props.loadingState.name === 'Loading') {
        return <div>Loading this account's transactions...</div>
    }
    if (props.loadingState.name === 'Error') {
        let msg = props.loadingState.message;
        let text = `Error while loading transactions${msg? ': ' + msg : ''}.`;
        return <div className="error-text">
            {text}
            </div>
    }
    if (props.transactions === null || props.transactions.length === 0) {
        return <div>No transactions yet!</div>;
    }
    let table = new Table([
        new DateFieldColumn('Time', 'transactionTime'), 
        new FieldColumn('Value', 'value', (e: any) => ((e.value as number) / 100).toFixed(2)), 
        new DollarsFieldColumn('Balance', 'balance'), 
        new FieldColumn('Type', 'businessEnds'), 
        new FieldColumn('Note', 'note'), 
    ], props.transactions);
    let visitor = new ReactTableVisitor();
    table.accept(visitor);
    console.log(table, 'accepting', visitor);
    return visitor.render();
}

/*function getRows(transactions: Transaction[]) {
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
    let hoursMinutes = `${padZeroes(2, time.getHours())}:${padZeroes(2, time.getMinutes())}`;
    return `${date} ${hoursMinutes}`;
}*/
