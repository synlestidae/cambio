import {Section} from './section';
import * as React from 'react';
import {ReactSectionVisitor} from './react_section_visitor';

export class ReactFormVisitor {
    protected components: JSX.Element[] = [];
    private sectionVisitor: ReactSectionVisitor;
    private onChange: Function;
    private onSubmit: Function = () => {};

    constructor(onChange: Function, sectionVisitor?: ReactSectionVisitor) {
        this.sectionVisitor = sectionVisitor || new ReactSectionVisitor();
        this.onChange = onChange;
    }

    public visitTitle(title: string): void {
        this.components.push(<h4>{title}</h4>);
    }

    public visitDescription(description: string): void {
        this.components.push(<p>{description}</p>);
    }

    public visitSection(section: Section): void {
        section.accept(this.sectionVisitor);         
        this.components.push(this.sectionVisitor.render());
    }

    public visitOnSubmit(onSubmit: Function) {
        this.onSubmit = onSubmit;
        this.components.push(<input className="form-control" type="submit" value="Submit"></input>);
    }

    public render(): JSX.Element {
        const onSubmit = () => {
            this.onSubmit();
            return false;
        };
        return <form onChange={() => this.onChange()} onSubmit={onSubmit}>
          {this.components}    
        </form>;
    }
}
