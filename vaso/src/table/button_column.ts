import {Column} from './column';

export class ButtonColumn<E> extends Column<E> {
    public readonly onClick: (e: E) => void
    public readonly buttonText: string; 
    public readonly sortable = false;

    constructor(title: string, buttonText: string, onClick: (e: E) => void) {
        super(title); 
        this.buttonText = buttonText;
        this.onClick = onClick;
    }
}
