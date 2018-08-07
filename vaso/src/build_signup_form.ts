import * as React from "react";
import {CalendarDate} from './domain/calendar_date';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';
import {Form} from './form/form';
import {Section} from './form/section';
import {SingleForm} from './form/single_form';
import {TextFieldElement} from './form/text_field_element';
import {ReadonlyFieldElement} from './form/readonly_field_element';
import {FieldElement} from './form/field_element';

export type FormState = 'LoginInfo' | 'PersonalDetails' | 'ConfirmationCode';

interface PartialSignupFormProps {
    actions: ActionCreators;
}

export function buildSignupForm(props: {signupState: SignupState} & PartialSignupFormProps): Form {
    let elems: FieldElement[];
    let header: string;
    let signupState = props.signupState;
    let formState = props.signupState.formState;

    if (formState == 'LoginInfo') {
        header = 'Choose your login credentials';
        elems = [
            new TextFieldElement('emailAddress', signupState, 'Email Address', 'email'),
            new TextFieldElement('password', signupState, 'Password', 'password'),
            new TextFieldElement('confirmedPassword', signupState, 'Password', 'password'),
        ];
    } else if (formState == 'PersonalDetails') {
        header = 'Enter your personal details';
        elems = [
            new TextFieldElement('givenName', signupState, 'Given name', 'given-name'),
            new TextFieldElement('familyName', signupState, 'Family name', 'family-name'),
            new TextFieldElement('familyName', signupState, 'Family name', 'family-name'),
            new TextFieldElement('addressLine1', signupState, 'Address line 1', 'address-line-1'),
            new TextFieldElement('addressLine2', signupState, 'Address line 2', 'address-line-2'),
            new TextFieldElement('postCode', signupState, 'Post code', 'postal-code'),
            new TextFieldElement('city', signupState, 'City or town', 'city'),
            new ReadonlyFieldElement('country', 'Country', 'country'),
        ];
    } else if (formState == 'ConfirmationCode') {
        header = `Enter the confirmation code that was emailed to ${signupState.emailAddress}.`;
        elems = [
            new TextFieldElement('confirmationCode', signupState, 'Confirmation code')
        ];
    } else {
        throw new Error(`Unknown form state: ${formState}`);
    }

    return new SingleForm([new Section(elems)], header);
}
