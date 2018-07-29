import {Section} from './section';

export abstract class Form {
    public readonly title: string|null;
    public readonly description: string|null;
    private readonly sections: Section[];
    private _onChange: Function|null = null;
    //private _onSubmit: Function|null = null;
    //private _onCancel: Function|null = null;

    set onChange(func: Function) {
        this._onChange = func;
    }

        /*set onSubmit(func: Function) {
        this._onSubmit = func;
    }

    set onCancel(func: Function) {
        this._onCancel = func;
    }*/

    constructor(sections: Section[], title?: string, description?: string) {
        this.sections = sections;
        this.title = title || null;
        this.description = description || null;
    }

    public getSections(): Section[] {
        return this.sections;
    }

    public isValid(): boolean {
        for (let section of this.sections) {
            if (!section.isValid()) {
                return false;
            }
        }
        return true;
    }

    public callOnChange() {
        if (this._onChange) {
            this._onChange.call(this);
        }
    }

    /*public callOnSubmit() {
        if (this._onSubmit) {
            this._onSubmit.call(this);
        }
    }

    public callOnCancel() {
        if (this._onSubmit) {
            this._onCancel.call(this);
        }
    }*/
}
