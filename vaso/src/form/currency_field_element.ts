import {TextFieldElement} from './text_field_element';

export class CurrencyFieldElement extends TextFieldElement {
    public decimalPlaces: number = 0;

    constructor(property: string, fieldObject: Object, label: string, name?: string) {
        super(property, fieldObject, label, name);
    }

    public onBlur(): void {
        let value = this.getValue();
        let [_, integerPartStr, fractionalPartStr]  = /(\d*).?(\d+)?/.exec(value);
        this.setValue(`${parseInt(integerPartStr) || 0}.${(fractionalPartStr || '0').substr(0, this.decimalPlaces)}`);
    }

    public getType() {
        return 'number';
    }
}
