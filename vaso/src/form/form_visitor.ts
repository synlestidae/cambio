import {Section} from './section';

export interface FormVisitor {
    visitTitle(title: string): void;
    visitDescription(description: string): void;
    visitSection(section: Section): void;
    visitOnSubmit(onSubmit: Function): void;
}
