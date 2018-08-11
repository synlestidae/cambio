import {Section} from './section';
import * as React from 'react';
import {ReactSectionVisitor} from './react_section_visitor';

export class ReactFormVisitor {
    private components: JSX.Element[] = [];
    private sectionVisitor: ReactSectionVisitor;
    private onChange: Function;
    private onSubmit: Function = () => {};

    constructor(sectionVisitor: ReactSectionVisitor, onChange: Function) {
        this.sectionVisitor = sectionVisitor;
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
