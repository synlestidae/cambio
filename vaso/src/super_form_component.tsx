import {SuperForm} from './form/super_form';
import {FormButton} from './form/form_button';
import {FormComponent} from './form_component';
import * as React from "react";
import {SignupStateName} from './flux/state/signup_state_name';
import {LoadingState} from './flux/state/loading_state';

interface SuperFormComponentProps {
    form: SuperForm<SignupStateName>
}

export function SuperFormComponent(props: SuperFormComponentProps): JSX.Element {
    let nextButton = props.form.getNextButton();
    let prevButton = props.form.getPreviousButton();
    nextButton.loading = props.form.loadingState.name === 'Loading';
    return <div>
        <FormComponent form={props.form.getCurrentForm()} state={props.form.loadingState}/>
        <div className="form-row form-buttons">
          {prevButton? <FormButtonComponent {...prevButton} /> : null}
          <FormButtonComponent {...nextButton} />
        </div>
    </div>;
}

function FormButtonComponent(props: FormButton) {
    return <button onClick={() => props.action()} className="btn btn-primary width-initial non-touching-button" disabled={props.disabled}>
      {props.loading? <i className="fa fa-spinner fa-spin"></i>  : props.text}
    </button>
}
