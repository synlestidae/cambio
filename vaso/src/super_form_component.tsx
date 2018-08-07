import {SuperForm} from './form/super_form';
import {FormButton} from './form/form_button';
import {FormComponent} from './form_component';
import * as React from "react";
import {SignupStateName} from './flux/state/signup_state_name';

interface SuperFormComponentProps {
    form: SuperForm<SignupStateName>
}

export function SuperFormComponent(props: SuperFormComponentProps): JSX.Element {
    let prevButton = props.form.getPreviousButton();
    return <div>
        <FormComponent form={props.form.getCurrentForm()}/>
        <div className="form-row">
          <FormButtonComponent {...props.form.getNextButton()} />
          {prevButton? <FormButtonComponent {...prevButton} /> : null}
        </div>
    </div>;
}

function FormButtonComponent(props: FormButton) {
    return <button onClick={() => props.action()} className="btn btn-primary btn-block width-initial" disabled={props.disabled}>
      {props.text}
    </button>
}
