import {TextFieldElement} from './text_field_element';

export class OptionFieldElement extends TextFieldElement {
    public options: Option[];

    constructor(property: string, fieldObject: Object, label: string, options: Option[], name?: string) {
        super(property, fieldObject, label, name);
        this.options = options;
    }

    public getType() {
        return 'option';
    }
}

interface Option {
    value: string;
    label: string;
}
