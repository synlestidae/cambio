import {FieldColumn} from './field_column';

export class DollarsFieldColumn<E> extends FieldColumn<E> {
    constructor(title: string, field: string) {
        super(title, field);
    }

    public format(row: E): string {
        let value = (row as any)[this.field];
        if (typeof value === 'number') {
            return `$${(value / 100).toFixed(2)}`;
        }
        throw new Error(`Type of field '${this.field}' in dollar column was ${typeof value}, expected number`);
    }
}
