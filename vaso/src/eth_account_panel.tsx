import * as React from 'react';
import {Table} from './table/table';
import {FieldColumn} from './table/field_column';
import {ReactTableVisitor} from './table/react_table_visitor';
import {CryptoAccountTableVisitor} from './table/crypto_account_table_visitor';
import {CryptoAccount} from './domain/crypto_account';

import {TextFieldElement} from './form/text_field_element';
import {Section} from './form/section';
import {SingleForm} from './form/single_form';

import {ActionCreators} from './flux/action_creators';

interface EthAccountPanelProps {
    accounts: CryptoAccount[],
    editingAccount: CryptoAccount|null,
    actions: ActionCreators
}

export function EthAccountPanel(props: EthAccountPanelProps) {
    let table = getTable(props.accounts);
    let visitor = new CryptoAccountTableVisitor(
        () => props.actions.beginNewCryptoAccount(),
        () => props.actions.setNewCryptoAccount(props.editingAccount),
        props.actions
    );
    visitor.emptyMessage = 'You don\'t have any Ethereum accounts yet.';
    if (props.editingAccount) {
        visitor.newAccountForm = getForm(props.editingAccount, 
            () => props.actions.saveNewCryptoAccount(props.editingAccount),
            () => props.actions.discardNewCryptoAccount()
        );
    }
    table.accept(visitor);

    return <div className="account-container">
        {visitor.render()}
    </div>;
}

function getTable(accounts: CryptoAccount[]) {
    let columns = [
        new FieldColumn<CryptoAccount>('Account name', 'name'),
        new FieldColumn<CryptoAccount>('Address', 'address'),
        new FieldColumn<CryptoAccount>('Balance', 'balance'),
    ];
    return new Table(columns, accounts);
}

function getForm(account: CryptoAccount, onSave: () => void, onCancel: () => void): SingleForm {
    const fields = [
        new TextFieldElement('name', account, 'Name'),
        new TextFieldElement('address', account, 'Ethereum address')
    ];
    const noop = () => {};
    const form = new SingleForm(
        [new Section(fields)],
        onSave,
        'Add your Ethereum account'
    );
    form.onCancel = onCancel;
    return form;
}
