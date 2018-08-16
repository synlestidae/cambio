import {CryptoAccount} from '../domain/crypto_account';
import {ReactTableVisitor} from '../table/react_table_visitor';
import * as React from 'react';
import {Column} from '../table/column';
import {ClipboardCopy} from '../clipboard_copy';
import {SingleForm} from '../form/single_form';
import {SingleFormVisitor} from './../form/single_form_visitor';
import {EditBox} from './edit_box';
import {ActionCreators} from '../flux/action_creators';

export class CryptoAccountTableVisitor extends ReactTableVisitor<CryptoAccount> {
    private footer: JSX.Element|null = null;

    private onEdit: () => void;
    private onChange: () => void;
    private actions: ActionCreators;

    public newAccountForm: SingleForm|null;

    constructor(onEdit: () => void, onChange: () => void, actions: ActionCreators) {
        super();
        this.onEdit = onEdit;
        this.onChange = onChange;
        this.actions = actions;
    }

    public visitCell(rowValue: CryptoAccount, column: Column<CryptoAccount>) {
        let value = (rowValue as any)[column.field] || '';
        let cell: JSX.Element;; 

        if (column.field === 'address') {
            if (value.startsWith('0x')) {
                value = value.substring(2);
            }
            cell = <div style={{fontFamily: 'monospace'}}>{value} <ClipboardCopy value={value}/></div>;
        } else if (column.field === 'balance') {
            cell = <span>--</span>;
        } else if (column.field === 'name') {
            cell = <EditBox value={value} onDone={(newName: string) => this.actions.changeCryptoAccountName(rowValue, newName)}/> 
        } else {
            cell = <span>{value}</span>;
        }

        this.currentRow.push(<td key={column.field}>{cell}</td>);
    }

    public visitFooter() {
        this.flushRow();
        if (this.newAccountForm) {
            let visitor = new SingleFormVisitor(this.onChange);
            this.newAccountForm.accept(visitor);
            this.footer = visitor.render();
        } else {
            this.footer = <button onClick={this.onEdit} className="btn btn-primary">Add account</button>
        }
    }

    public render(): JSX.Element {
        return <table style={{width: '100%'}} className="table-component">
          <tfoot>
            <tr>
              <td colSpan={this.headers.length} className="crypto-account-footer">
                {this.footer}
              </td>
            </tr>
          </tfoot>
          <tbody>
            <tr key={-1}>
              {this.headers}
            </tr>
            {this.rows.length? this.rows : [this.emptyRow()]}
          </tbody>
        </table>;
    }
}
