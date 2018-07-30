import {TextFieldElement} from './text_field_element';

export class CurrencyFieldElement extends TextFieldElement {
    public decimalPlaces: number = 2;

    constructor(property: string, fieldObject: Object, label: string, name?: string) {
        super(property, fieldObject, label, name);
    }

    public onBlur(): void {
        let newValue = this.cleanDecimal(this.getValue());
        this.setValue(newValue);
    }

    private cleanDecimal(val: string): string {
        let [_, integerPartStr, fractionalPartStr]  = /(\d*).?(\d+)?/.exec(val);
        let integerPart = parseInt(integerPartStr) || 0;
        let fractionalPart = parseInt(fractionalPartStr) || 0;
        if (this.decimalPlaces <= 0) {
            return integerPart.toString();
        }
        return `${integerPart}.${fractionalPart.toString().substring(0, this.decimalPlaces)}`;
    }

    public getType() {
        return 'number';
    }

}
