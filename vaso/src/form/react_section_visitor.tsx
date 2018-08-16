import * as React from 'react';
import {FieldElement} from './field_element';
import {OptionFieldElement} from './option_field_element';
import {SectionVisitor} from './section_visitor';

export class ReactSectionVisitor implements SectionVisitor {
    private components: JSX.Element[] = [];

    public visitTitle(title: string): void {
        this.components.push(<p>{title}</p>);
    }

    public visitField(fieldElem: FieldElement): void {
        let validation: JSX.Element|null = null;
        let inputElem: JSX.Element;
        if (fieldElem instanceof OptionFieldElement) {
            inputElem = this.renderOptionField(fieldElem);
        } else {
            inputElem = this.renderInputField(fieldElem);
        }
        if (fieldElem.getValidationMessage() !== null && fieldElem.isDirty()) {
            validation = <p className="validation-message">{fieldElem.getValidationMessage()}</p>;
        }
        let required = fieldElem.isRequired()? <span className="required">*</span> : null;
        let validationClass = (fieldElem.isRequired() && fieldElem.getValidationMessage() && fieldElem.isDirty() && 'invalid-input') || '';
        let className = `form-control ${validationClass}`;

        let element = <div className="form-group">
            <label className="form-label" htmlFor={fieldElem.getName()}>{fieldElem.getLabel()} {required}</label>
            {inputElem}
            {validation}
        </div>;
        this.components.push(element);
    }

    private renderOptionField(fieldElem: OptionFieldElement): JSX.Element {
        let options = fieldElem.options.map((option: any) => <option value={option.value}>{option.label}</option>);
        return <select className="form-control" onChange={(e: any) => fieldElem.setValue(e.target.value)} value={fieldElem.getValue()}>
            <option value="">--</option>
            {options}
        </select>;
    }

    private renderInputField(fieldElem: FieldElement) {
        let val = fieldElem.getValue();
        return <input 
            className="form-control" 
            type={fieldElem.getType()} 
            name={fieldElem.getName()} 
            disabled={fieldElem.isDisabled()}
            value={val} 
            min={0}
            step={0.01}
            onChange={(e: any) => fieldElem.setValue(e.target.value)}
            onFocus={() => fieldElem.onFocus()} 
            onBlur={() => fieldElem.onBlur()}>
        </input>
    }

    public render() {
        return <fieldset>
            {this.components}
        </fieldset>;
    }
}
