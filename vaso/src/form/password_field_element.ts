import {TextFieldElement} from './text_field_element';

export class PasswordFieldElement extends TextFieldElement {
    public getType(): string {
        return 'password';
    }

    public isValid(): boolean {
        return this.getValue().length >= 8;
    }
}
