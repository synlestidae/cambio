import {Form} from './form';;
import {Section} from './section';
import {LoadingState} from '../flux/state/loading_state';
import {SingleFormVisitor} from './single_form_visitor';

export class SingleForm {
    public form: Form;
    public loadingState = new LoadingState();
    public onCancel: () => {};

    constructor(sections: Section[], onSubmit: Function, title: string) {
        this.form = new Form(sections, onSubmit, title);
    }

    public accept(visitor: SingleFormVisitor) {
        this.form.accept(visitor);
        visitor.visitOnCancel(this.onCancel);
    }
}
