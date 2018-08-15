import {CryptoAccount} from '../domain/crypto_account';
import {TableVisitor} from '../table/table_visitor';
import * as React from 'react';
import {Column} from '../table/column';
import {ClipboardCopy} from '../clipboard_copy';
import {SingleForm} from '../form/single_form';
import {SingleFormVisitor} from './../form/single_form_visitor';

export class CryptoAccountTableVisitor implements TableVisitor<CryptoAccount> {
    private headers: JSX.Element[] = [];
    private rows: JSX.Element[] = [];
    private currentRow: JSX.Element[] = [];
    private footer: JSX.Element|null = null;
    private onEdit: () => void;
    private onChange: () => void;

    public newAccountForm: SingleForm|null;

    constructor(onEdit: () => void, onChange: () => void) {
        this.onEdit = onEdit;
        this.onChange = onChange;
    }

    public visitColumnHeader(h: Column<CryptoAccount>) {
        this.headers.push(<th key={h.field}>
             <span>
                {h.title}
             </span>
             &nbsp;
            </th>);
    }

    public visitBody(rows: CryptoAccount[], columns: Column<CryptoAccount>[]) {
    }

    public visitRow(rowValue: CryptoAccount, columns: Column<CryptoAccount>[]) {
        this.flushRow();
    }

    public visitCell(rowValue: CryptoAccount, column: Column<CryptoAccount>) {
        let value = (rowValue as any)[column.field] || '';
        let cell: JSX.Element;; 

        if (column.field === 'address') {
            if (value.startsWith('0x')) {
                value = value.substring(2);
            }
            cell = <div style={{fontFamily: 'monospace'}}>{value} <ClipboardCopy value={value}/></div>;
        } else if (column.field == 'balance') {
            cell = <span>--</span>;
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

    private flushRow() {
        this.rows.push(<tr key={this.rows.length}>{this.currentRow}</tr>);
        this.currentRow = [];
    }

    private emptyRow(): JSX.Element {
        return <tr className="empty-row">
          <td colSpan={this.headers.length}>
            <em>No Ethereum accounts yet.</em>
          </td>
        </tr>;
    }
}
