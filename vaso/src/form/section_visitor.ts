import {FieldElement} from './field_element';

export interface SectionVisitor {
    visitTitle(title: string): void;
    visitField(field: FieldElement): void;
}
