export abstract class Column<E> {
    public readonly title: string;
    public readonly field: string;
    public readonly sortable: boolean = true;

    constructor(title: string) {
        this.title = title;
        this.field = title;
    }
}

