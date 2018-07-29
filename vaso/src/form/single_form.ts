import {Form} from './form';;
import {Section} from './section';

export class SingleForm extends Form {
    constructor(sections: Section[], title: string, onSubmit: (obj: any) => void, onCancel: (obj: any) => void) {
        super(sections, title);
    }
}
