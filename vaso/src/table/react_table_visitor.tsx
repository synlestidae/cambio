import * as React from "react";
import {TableVisitor} from './table_visitor';
import {Column} from './column';

export class ReactTableVisitor implements TableVisitor<any> {
    private headers: JSX.Element[] = [];
    private rows: JSX.Element[] = [];

    private currentRow: JSX.Element[] = [];
    private key = 0;

    public visitBody() {
    }

    public visitColumnHeader(h: Column<any>) {
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

    public visitRow(rowValue: any, columns: Column<any>[]): void {
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

    public visitCell(rowValue: any, column: Column<any>): void {
        let cell: string = rowValue[column.field] || '';
        this.currentRow.push(<td key={column.field}>{cell}</td>);
    }

    public visitFooter() {
    }

    private flushRow() {
        if (this.currentRow.length) {
            this.rows.push(<tr key={this.key++}>{this.currentRow}</tr>);
        }
        this.currentRow = [];
    }

    private emptyRow(): JSX.Element {
        return <tr className="empty-row">
          <td colSpan={this.headers.length}>
            <em>No rows yet.</em>
          </td>
        </tr>;
    }
}
