import * as React from 'react';
import {Table} from './table/table';
import {FieldColumn} from './table/field_column';
import {ReactTableVisitor} from './table/react_table_visitor';
import {CryptoAccountTableVisitor} from './table/crypto_account_table_visitor';
import {CryptoAccount} from './domain/crypto_account';

interface EthAccountPanelProps {
    accounts: CryptoAccount[]
}

export function EthAccountPanel(props: EthAccountPanelProps) {
    let columns = [
        new FieldColumn<CryptoAccount>('Account name', 'name'),
        new FieldColumn<CryptoAccount>('Address', 'address'),
    ];
    let table = new Table(columns, props.accounts);
    let visitor = new CryptoAccountTableVisitor();
    table.accept(visitor);
    return <div className="account-container">
        {visitor.render()}
    </div>;
}
