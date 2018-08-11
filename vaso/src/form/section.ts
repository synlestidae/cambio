import {FieldElement} from './field_element';
import {SectionVisitor} from './section_visitor';

export class Section {
    public readonly title: string|null;    
    private readonly fields: FieldElement[];
    
    constructor(fields: FieldElement[], title?: string) {
        this.fields = fields;
        this.title = title || null;
    }

    public getElements(): FieldElement[] {
        return this.fields;
    }

    public accept(visitor: SectionVisitor) {
        if (this.title) {
            visitor.visitTitle(this.title);
        }
        for (let field of this.fields) {
            visitor.visitField(field);
        }
    }
}
