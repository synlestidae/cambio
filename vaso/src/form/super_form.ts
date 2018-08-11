import {Form} from './form';
import {FormButton} from './form_button';
import {LoadingState} from '../flux/state/loading_state';

export class SuperForm<E> {
    private currentForm: E;
    private screens: FormScreen<E>[] = [];
    public loadingState = new LoadingState();
    public onChange: Function = () => {};

    constructor(currentForm: E) {
        this.currentForm = currentForm;
    }

    public addScreen(form: Form, name: E, nextButton: FormButton, previousButton?: FormButton) {
        this.screens.push({
            name: name,
            form: form,
            next: nextButton,
            prev: previousButton
        });
    }

    public getCurrentForm(): Form|null {
        let s = this.getScreen();
        return s && s.form;
    }

    public getNextButton(): FormButton|null {
        let s = this.getScreen();
        if (!(s && s.next)) {
            return null;
        }
        let button = s.next; 
        return button;
    }

    public getPreviousButton(): FormButton|null {
        let s = this.getScreen();
        return s && s.prev || null;
    }

    private getScreen(): FormScreen<E>|null {
        for (let s of this.screens) {
            if (s.name === this.currentForm) {
                return s;
            }
        }
        return null;
    }
}

interface FormScreen<E> {
    name: E;
    form: Form;
    next: FormButton;
    prev?: FormButton;
}
