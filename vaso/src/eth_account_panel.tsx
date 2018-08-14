import * as React from 'react';
import {Table} from './table/table';
import {FieldColumn} from './table/field_column';
import {ReactTableVisitor} from './table/react_table_visitor';

interface EthAccountPanelProps {
    columns: FieldColumn<any>[],
    rows: any[]
}

export function EthAccountPanel(props: EthAccountPanelProps) {
    let columns = [new FieldColumn<any>('Account name', 'name')];
    let rows: any[] = [];
    let table = new Table(columns, rows);
    let visitor = new ReactTableVisitor();

    return <div className="account-container">
        {visitor.render()}
    </div>;
}
