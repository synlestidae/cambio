import {Section} from './section';

export class Form {
    public readonly title: string|null;
    public readonly description: string|null;
    private readonly sections: Section[];

    constructor(sections: Section[], title?: string, description?: string) {
        this.sections = sections;
        this.title = title || null;
        this.description = description || null;
    }

    public getSections(): Section[] {
        return this.sections;
    }

    /*public isValid(): boolean {
        for (let section of this.sections) {
            if (!section.isValid()) {
                return false;
            }
        }
        return true;
    }*/
}
