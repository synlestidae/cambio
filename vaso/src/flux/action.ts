export interface Action {
    name: string,
    value: string|null,
    payload: any|null
}

export class BasicAction implements Action {
    public readonly name: string;
    public readonly value: string; 
    public readonly payload: any|null = null;

    constructor(name: string, value?: string) {
        this.name = name;
        this.value = value || null;
    }
}
