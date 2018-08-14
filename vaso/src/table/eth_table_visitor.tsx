
export class interface TableVisitor<E> {
    visitColumnHeader(h: Column<E>): void;
    visitBody(rows: E[], columns: Column<E>): void;
    visitRow(rowValue: E, columns: Column<E>[]): void;
    visitCell(rowValue: E, column: Column<E>): void;
    visitFooter(): void;
}
