export interface FieldElement {
    getValue(): string;
    setValue(val: string): void;
    getLabel(): string;
    getName(): string|null;
    onFocus(): void;
    onBlur(): void;
    isDisabled(): boolean;
    isRequired(): boolean;
    getValidationMessage(): string|null;
    getType(): string;
}
