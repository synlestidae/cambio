import {CryptoAccount} from '../domain/crypto_account';
import {TableVisitor} from '../table/table_visitor';
import * as React from 'react';
import {Column} from '../table/column';

export class CryptoAccountTableVisitor implements TableVisitor<CryptoAccount> {
    private headers: JSX.Element[] = [];
    private rows: JSX.Element[] = [];
    private currentRow: JSX.Element[] = [];

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
        const value = (rowValue as any)[column.field].toString();
        this.currentRow.push(<td>{value}</td>);
    }

    public visitFooter() {
        this.flushRow();
    }

    public render(): JSX.Element {
        return <table style={{width: '100%'}} className="table-component">
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
    }

    private emptyRow(): JSX.Element {
        return <tr className="empty-row">
          <td colSpan={this.headers.length}>
            <em>No rows yet.</em>
          </td>
        </tr>;
    }

}
