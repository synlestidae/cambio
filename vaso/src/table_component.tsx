import * as React from "react";

type SortCallback = (field: string) => void;

export abstract class Column<E> {
    public title: string;
    public sortable: boolean = true;

    constructor(title: string) {
        this.title = title;
    }

    public abstract cell(item: E): string;
}

export class OperationColumn<E> extends Column<E> {
    public readonly field: string;
    public readonly cb: (o: E) => void;
    public readonly name: string;

    constructor(name: string, field: string, cb: (o: E) => void) {
        super('');
        this.sortable = false;
        this.field = field;
        this.cb = cb;
        this.name = name;
    }

    public cell(item: E): string {
        return this.name;
    }
}

export class FieldColumn<E> extends Column<E> {
    private cellFn: (c: E) => string;
    private field: string;

    constructor(title: string, field: string, cell: (c: E) => string) {
        super(title);
        this.cellFn = cell;
        this.field = field;
    }

    public cell(item: E) {
        return this.cellFn(item);
    }
}

interface TableComponentProps<E> {
    columns: Column<E>[],
    rows: E[],
    sortCB?: (s: string) => void,
    emptyMessage?: string
}

function getColumnHeader<E>(h: Column<E>, i: number, sortCB?: (field: string) => void) {
   return <th key={i}>
     <span>
        {h.title}
     </span>
        &nbsp;
     <span>
        {h.sortable? <i className="fas fa-sort clickable" onClick={() => sortCB && sortCB(h.title)}></i> : null}
      </span>
    </th>;
}

function getRow<E>(headers: Column<E>[], row: E, i: number) {
    let columns = [];
    let key = 0;
    for (let header of headers) {
        if (header instanceof OperationColumn) {
            let cb = (header as OperationColumn<E>).cb;
            columns.push(<td key={key}>
                <button className="btn btn-sm" onClick={() => cb(row)}>{header.cell(row)}</button>
            </td>);
        } else {
            columns.push(<td key={key}>{header.cell(row)}</td>);
        }
        key++;
    }
    return <tr key={i}>{columns}</tr>;
}

export function TableComponent<E>(props: TableComponentProps<E>) {
    let columns = props.columns.map((h: Column<E>, i: number) => getColumnHeader(h, i, props.sortCB));
    let rows = props.rows.map((r: E, i: number) => getRow(props.columns, r, i));
    let emptyRow = [
        <tr className="empty-row" key={0}>
          <td colSpan={columns.length}>
            <em>{props.emptyMessage}</em>
          </td>
        </tr>
    ];
    return <table style={{width: '100%'}} className="table-component">
      <tbody>
        <tr key={-1}>
          {columns}
        </tr>
        {rows.length === 0 && props.emptyMessage? emptyRow : rows}
      </tbody>
    </table>
}
