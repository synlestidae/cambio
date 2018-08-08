import {TextFieldElement} from './text_field_element';

export class RequiredTextFieldElement extends TextFieldElement {
    /*constructor(property: string, fieldObject: Object, label: string, msg: string) {
        super(property, fieldObject, label, msg);
    }*/
    public isRequired() {
        return true;
    }

    public isValid(): boolean {
        return Boolean(this.getValue());
    }

    public getValidationMessage(): string|null {
        return this.isValid()? null : 'This field cannot be empty';
    }
}
