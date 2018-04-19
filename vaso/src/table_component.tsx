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
    columns: FieldColumn<E>[],
    rows: E[],
    sortCB?: (s: string) => void
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

function getRow<E>(headers: FieldColumn<E>[], row: E, i: number) {
    let columns = [];
    let key = 0;
    for (let header of headers) {
        let contents: string;
        contents = header.cell(row);
        columns.push(<td key={key}>{contents}</td>);
        key++;
    }
    return <tr key={i}>{columns}</tr>;
}

export function TableComponent<E>(props: TableComponentProps<E>) {
    let columns = props.columns.map((h: Column<E>, i: number) => getColumnHeader(h, i, props.sortCB));
    let rows = props.rows.map((r: E, i: number) => getRow(props.columns, r, i));
    return <table style={{width: '100%'}} className="table-component">
      <tbody>
        <tr>
          {columns}
        </tr>
        {rows}
      </tbody>
    </table>
}
