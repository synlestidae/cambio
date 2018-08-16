import {Column} from './column';

export class FieldColumn<E> extends Column<E> {
    public readonly field: string;
    private cell: null|((e: E) => string) = null;

    constructor(title: string, field: string, cell?: (item: E) => string) {
        super(title);
        this.field = field;
        this.cell = cell || null;
    }

    public format(val: E): string {
        if (this.cell) {
            return this.cell(val);
        }
        return super.format(val);
    }
}
