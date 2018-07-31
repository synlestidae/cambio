import {Section} from './section';
import {LoadingState} from '../flux/state/loading_state';

export abstract class Form {
    public readonly title: string|null;
    public readonly description: string|null;
    public state: LoadingState = new LoadingState();

    private readonly sections: Section[];
    private _onChange: Function|null = null;

    set onChange(func: Function) {
        this._onChange = func;
    }

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
}
