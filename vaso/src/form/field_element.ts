export interface FieldElement {
    getValue(): string;
    setValue(val: string): void;
    getLabel(): string;
    getName(): string|null;
    getValidationMessage(): string|null;
    getType(): string;
}
