import {Column} from './column';
import {TableVisitor} from './table_visitor';

export class Table<E> {
    public columns: Column<E>[] = [];
    public rows: E[] = [];

    constructor(columns: Column<E>[], rows: E[]) {
        this.columns = columns;
        this.rows = rows;
    }

    public accept(visitor: TableVisitor<E>): void {
        for (let column of this.columns) {
            visitor.visitColumnHeader(column);
        }
        visitor.visitBody(this.rows, this.columns);
        for (let row of this.rows) {
            visitor.visitRow(row, this.columns);
            for (let column of this.columns) {
                visitor.visitCell(row, column);
            }
        }
        visitor.visitFooter();
    }
}
