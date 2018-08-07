import {TextFieldElement} from './text_field_element';

export class CodeFieldElement extends TextFieldElement {
    public getType(): string {
        return 'password';
    }

    public setValue(val: string) {
        val = /(\d{1,5})/.exec(val)[1];
        super.setValue(val || '');
    }
}
