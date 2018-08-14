import {Column} from './column';
import {Row} from './row';

export interface TableVisitor<E> {
    visitColumnHeader(h: Column<E>): void;
    visitRow(rowValue: E, columns: Column<E>[]): void;
}
