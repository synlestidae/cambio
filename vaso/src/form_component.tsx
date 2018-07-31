import * as React from "react";
import {Form} from './form/form';
import {Section} from './form/section';
import {FieldElement} from './form/field_element';

interface FormComponentProps {
    form: Form;
    onSubmit?: () => void;
    onCancel?: () => void;
}

export function FormComponent(props: FormComponentProps) {
    let form = props.form;
    let title = form.title? <div>{form.title}</div> : null; 
    let formDisabled = form.state.name === 'Loading';
    let fields = form.getSections().map(function(section: Section, i: number) {
        return <FieldSetComponent title={section.title} elements={section.getElements()} key={i} formDisabled={formDisabled}>
            </FieldSetComponent>;
    });
    const preventSubmit = () => false;

    let error: JSX.Element|null;
    if (form.state.name === 'Error') {
        error = <div className="error-text">{form.state.message || 'An error occurred while submitting.'}</div>
    }

    const onClickSubmit = function() {
        props.onSubmit();
        return false;
    };

    const onClickCancel = function() {
        props.onCancel();
        return false;
    };

    let buttons = <section className="form-buttons side-by-side">
      {props.onCancel? <button className="form-control btn non-touching-button" onClick={onClickCancel}>Cancel</button> : null}
      {props.onSubmit? <button className="form-control btn btn-primary non-touching-button" onClick={onClickSubmit}>Submit</button> : null}
    </section>;

    return <form onChange={() => form.callOnChange()} onBlur={() => form.callOnChange()} onSubmit={preventSubmit}>
      {title}
      <section className="form-fields">
        {fields}
      </section>
      {buttons}
      {error}
    </form>;
}

function FieldSetComponent(props: {title: string, elements: FieldElement[], formDisabled: boolean}): JSX.Element {
    let fields = props
        .elements
        .map((f: FieldElement, i: number) => <FieldInputComponent key={i} fieldElement={f} formDisabled={props.formDisabled}></FieldInputComponent>);

    return <fieldset>
        {props.title? <legend>{props.title}</legend>: null}
        {fields}
    </fieldset>;
}

function FieldInputComponent(props: {fieldElement: FieldElement, formDisabled: boolean}): JSX.Element {
    let fieldElem = props.fieldElement;
    let validation: JSX.Element|null = null;
    if (fieldElem.getValidationMessage() !== null) {
        validation = <p className="validation-message">{fieldElem.getValidationMessage()}</p>;
    }
    let val = fieldElem.getValue();
    return <div className="form-group">
        <label htmlFor={fieldElem.getName()}>{fieldElem.getLabel()}</label>
        <input 
            className="form-control" 
            type={fieldElem.getType()} 
            name={fieldElem.getName()} 
            disabled={fieldElem.isDisabled() || props.formDisabled}
            value={val} 
            min={0}
            step={0.01}
            onChange={(e: any) => fieldElem.setValue(e.target.value)}
            onFocus={() => fieldElem.onFocus()} 
            onBlur={() => fieldElem.onBlur()}>
        </input>
        {validation}
    </div>;
}
