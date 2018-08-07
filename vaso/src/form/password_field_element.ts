import {TextFieldElement} from './text_field_element';

export class PasswordFieldElement extends TextFieldElement {
    public getType(): string {
        return 'password';
    }
}
