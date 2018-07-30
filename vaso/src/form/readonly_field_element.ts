import {TextFieldElement} from './text_field_element';

export class ReadonlyFieldElement extends TextFieldElement {
    constructor(value: string, label: string, name?: string) {
        super('value', {value: value}, label, name);
    }

    public isDisabled() {
        return true;
    }
}
