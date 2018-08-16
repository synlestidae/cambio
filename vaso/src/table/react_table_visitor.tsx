import * as React from "react";
import {TableVisitor} from './table_visitor';
import {Column} from './column';

export class ReactTableVisitor<E> implements TableVisitor<E> {
    public emptyMessage: string|null = '';

    protected headers: JSX.Element[] = [];
    protected rows: JSX.Element[] = [];
    protected currentRow: JSX.Element[] = [];

    private key = 0;

    public visitBody() {
    }

    public visitColumnHeader(h: Column<E>) {
        this.headers.push(<th key={h.field}>
             <span>
                {h.title}
             </span>
                &nbsp;
             <span>
                {h.sortable? <i className="fas fa-sort clickable" onClick={() => {}}></i> : null}
              </span>
            </th>
        );
    }

    public visitRow(rowValue: E, columns: Column<E>[]): void {
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

    public visitCell(rowValue: E, column: Column<E>): void {
        let cell: string = column.format(rowValue); //String((rowValue as any)[column.field]) || '';
        this.currentRow.push(<td key={column.field}>{cell}</td>);
    }

    public visitFooter() {
        this.flushRow();
    }

    protected flushRow() {
        if (this.currentRow.length) {
            this.rows.push(<tr key={this.key++}>{this.currentRow}</tr>);
        }
        this.currentRow = [];
    }

    protected emptyRow(): JSX.Element {
        if (this.emptyMessage) {
            return <tr className="empty-row">
              <td colSpan={this.headers.length}>
                <em>{this.emptyMessage}</em>
              </td>
            </tr>;
        }
        return null;
    }
}
