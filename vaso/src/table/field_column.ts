import {Column} from './column';

export class FieldColumn<E> extends Column<E> {
    public readonly field: string;
    private cell = (e: E) => String(e);

    constructor(title: string, field: string, cell?: (item: E) => string) {
        super(title);
        this.field = field;
        this.cell = cell || this.cell;
    }
}
