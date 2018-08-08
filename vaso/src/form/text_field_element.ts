import {FieldElement} from './field_element';

export class TextFieldElement implements FieldElement {
    private property: string;
    private fieldObject: Object;
    private label: string;
    private name: string;
    private dirty = false;
    public required: boolean = false;

    constructor(property: string, fieldObject: Object, label: string, name?: string) {
        this.property = property;
        this.fieldObject = fieldObject;
        this.label = label;
        this.name = name || null;
    }

    public getValue(): string {
        let val: any = (<any>this.fieldObject)[this.property];
        if (val !== undefined && val !== null) {
            return val.toString();
        }
        return '';
    }

    public setValue(val: string) {
        this.dirty = false;
        (<any>this.fieldObject)[this.property] = val;
    }

    public onFocus(): void {
    }

    public onBlur(): void {
        this.dirty = true;
    }

    public getLabel(): string {
        return this.label;
    }

    public getName(): string {
        return this.name;
    }

    public getValidationMessage(): string|null {
        return null;
    }

    public getType(): string {
        return 'text';
    }

    public isDisabled(): boolean {
        return false;
    }

    public isRequired(): boolean {
        return this.required;
    }

    public isDirty(): boolean {
        return this.dirty;
    }
}
