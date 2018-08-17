import * as React from 'react';
import {UserSettlement} from './domain/user_settlement';
import {Table} from './table/table';
import {FieldColumn} from './table/field_column';
import {DateFieldColumn} from './table/date_field_column';
import {ReactTableVisitor} from './table/react_table_visitor';

interface SettlementPanelProps {
    settlements: UserSettlement[]
}

export function SettlementPanel(props: SettlementPanelProps): JSX.Element {
    let table = new Table([
        new FieldColumn('Ether', 'sourceOrder', (e: UserSettlement) => e.sourceOrder.amountCrypto),
        new FieldColumn('Dollars', 'sourceOrder', (e: UserSettlement) => e.sourceOrder.amountFiat),
        new FieldColumn('Price', 'sourceOrder', (e: UserSettlement) => e.sourceOrder.formatPrice()),
        new FieldColumn('From address', 'fromAddress'),
        new FieldColumn('To address', 'toAddress'),
        new DateFieldColumn('Due on blockchain', 'dueOnBlockchainAt'),
    ], props.settlements);
    let visitor = new ReactTableVisitor();
    table.accept(visitor)
    return <div className="account-container">
        {visitor.render()}
    </div>;
}
