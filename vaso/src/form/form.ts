import {Section} from './section';
import {FormVisitor} from './form_visitor';

export class Form {
    public readonly title: string|null;
    public readonly description: string|null;
    private readonly sections: Section[];
    private onSubmit: () => void;

    constructor(sections: Section[], onSubmit: () => void, title?: string, description?: string) {
        this.sections = sections;
        this.title = title || null;
        this.description = description || null;
        this.onSubmit = onSubmit;
    }

    public getSections(): Section[] {
        return this.sections;
    }

    public accept(visitor: FormVisitor) {
        if (this.title) {
            visitor.visitTitle(this.title);
        }
        if (this.description) {
            visitor.visitDescription(this.description);
        }
        for (let section of this.sections) {
            visitor.visitSection(section);
        }
        visitor.visitOnSubmit(this.onSubmit);
    }
}
