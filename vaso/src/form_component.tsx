import * as React from "react";
import {Form} from './form/form';
import {Section} from './form/section';
import {FieldElement} from './form/field_element';

interface FormComponentProps {
    form: Form;
    onSubmit: () => void;
    onCancel: () => void;
}

export function FormComponent(props: FormComponentProps) {
    let form = props.form;
    let title = form.title? <div>{form.title}</div> : null; 
    let fields = form.getSections().map(function(section: Section, i: number) {
        return <FieldSetComponent title={section.title} elements={section.getElements()} key={i}>
            </FieldSetComponent>;
    });
    const preventSubmit = () => false;

    return <form onChange={() => form.callOnChange()} onBlur={() => form.callOnChange()} onSubmit={preventSubmit}>
      {title}
      <section className="form-fields">
        {fields}
      </section>
      <section className="form-buttons side-by-side">
        <button className="form-control btn non-touching-button" onClick={() => props.onCancel()}>Cancel</button>
        <button className="form-control btn btn-primary non-touching-button" onClick={() => props.onSubmit()}>Submit</button>
      </section>
    </form>;
}

function FieldSetComponent(props: {title: string, elements: FieldElement[]}): JSX.Element {
    let fields = props
        .elements
        .map((f: FieldElement, i: number) => <FieldInputComponent key={i} fieldElement={f}></FieldInputComponent>);

    return <fieldset>
        {props.title? <legend>{props.title}</legend>: null}
        {fields}
    </fieldset>;
}

function FieldInputComponent(props: {fieldElement: FieldElement}): JSX.Element {
    let fieldElem = props.fieldElement;
    let validation: JSX.Element|null = null;
    if (fieldElem.getValidationMessage() !== null) {
        validation = <p className="validation-message">{fieldElem.getValidationMessage()}</p>;
    }
    return <div className="form-group">
        <label htmlFor={fieldElem.getName()}>{fieldElem.getLabel()}</label>
        <input 
            className="form-control" 
            type={fieldElem.getType()} 
            name={fieldElem.getName()} 
            value={fieldElem.getValue()} 
            onChange={(e: any) => fieldElem.setValue(e.target.value)}
            onFocus={() => fieldElem.onFocus()} 
            onBlur={() => fieldElem.onBlur()}>
        </input>
        {validation}
    </div>;
}
