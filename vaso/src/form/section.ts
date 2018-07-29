import {FieldElement} from './field_element';

export class Section {
    public readonly title: string|null;    
    private readonly elements: FieldElement[];
    
    constructor(elements: FieldElement[], title?: string) {
        this.elements = elements;
        this.title = title || null;
    }

    public getElements(): FieldElement[] {
        return this.elements;
    }

    public isValid(): boolean {
        for (let element of this.elements) {
            if (element.getValidationMessage() !== null) {
                return false;
            }
        }
        return true;
    }
}
