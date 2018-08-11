import * as React from "react";
import {Form} from './form/form';
import {Section} from './form/section';
import {FieldElement} from './form/field_element';
import {LoadingState} from './flux/state/loading_state';

interface FormComponentProps {
    form: Form;
    state: LoadingState;
    onSubmit?: () => void;
    onCancel?: () => void;
}

export function FormComponent(props: FormComponentProps) {
    let form = props.form;
    let title = form.title? <div>{form.title}</div> : null; 
    let formDisabled = props.state.name === 'Loading';
    let fields = form.getSections().map(function(section: Section, i: number) {
        return <FieldSetComponent title={section.title} elements={section.getElements()} key={i} formDisabled={formDisabled}>
            </FieldSetComponent>;
    });
    const preventSubmit = () => false;

    let error: JSX.Element|null;
    if (props.state.name === 'Error') {
        error = <div className="error-text">{props.state.message || 'An error occurred while submitting.'}</div>
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

    return <form onSubmit={preventSubmit}>
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
    return null;
}
