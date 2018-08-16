import {padZeroes} from '../pad_zeroes';
import {FieldColumn} from './field_column';

export class DateFieldColumn<E> extends FieldColumn<E> {
    constructor(title: string, field: string) {
        super(title, field);
    }

    public format(row: E): string {
        let value = (<any>row)[this.field];
        if (!(value instanceof Date)) {
            throw new Error(`Type of field '${this.field}' in date column was not Date`);
        }
        let date: Date = <Date>(row as any)[this.field];
        let datePart = `${date.getFullYear()}-${padZeroes(2, date.getMonth() + 1)}-${padZeroes(2, date.getDay())}`;
        let timePart = `${padZeroes(date.getHours(), 2)}:${padZeroes(date.getMinutes(), 2)}`;
        return `${datePart} ${timePart}`;
    }
}
