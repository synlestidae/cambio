import {Form} from './form';;
import {Section} from './section';

export class SingleForm extends Form {
    constructor(sections: Section[], title: string) {
        super(sections, title);
    }
}
