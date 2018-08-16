import {Section} from './section';
import * as React from 'react';
import {ReactSectionVisitor} from './react_section_visitor';

export class ReactFormVisitor {
    protected components: JSX.Element[] = [];
    protected buttons: JSX.Element[] = [];
    private sectionVisitor: ReactSectionVisitor;
    private onChange: () => void;

    constructor(onChange: () => void, sectionVisitor?: ReactSectionVisitor) {
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

    public visitOnSubmit(onSubmit: () => void) {
        let button = <input className="btn btn-primary width-initial non-touching-button" type="input" onClick={onSubmit} value="Submit"></input>
        this.buttons.push(button);
    }

    public render(): JSX.Element {
        return <form onChange={() => this.onChange()} onSubmit={() => false}>
          {this.components}    
          <div className="form-row form-buttons">
            {this.buttons}
          </div>
        </form>;
    }
}
