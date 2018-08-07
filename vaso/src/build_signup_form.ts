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
import {SuperForm} from './form/super_form';
import {SignupStateName} from './flux/state/signup_state_name';

interface PartialSignupFormProps {
    actions: ActionCreators;
}

export function buildSignupForm(props: {signupState: SignupState} & PartialSignupFormProps, onChange: Function): SuperForm<SignupStateName> {
    let elems: FieldElement[];
    let header: string;
    let signupState = props.signupState;

    let formState = props.signupState.formState;

    let superForm = new SuperForm(formState);
    let loginInfoForm = getLoginInfoForm(props.signupState);
    loginInfoForm.onChange = onChange;
    let personalDetailsForm = getPersonalDetailsForm(props.signupState);
    personalDetailsForm.onChange = onChange;
    let confirmationForm = getConfirmationForm(props.signupState);
    confirmationForm.onChange = onChange;

    superForm.addScreen(loginInfoForm, 'LoginInfo',  {
        text: 'Add personal details',
        action: () => props.actions.submitSignup(signupState.emailAddress, signupState.password),
        disabled: !loginInfoForm.isValid()
    });

    superForm.addScreen(personalDetailsForm, 'PersonalDetails', {
        text: 'Confirm email',
        action: () => props.actions.setSignupStatePage('PersonalDetails'),
        disabled: !personalDetailsForm.isValid()
    });

    superForm.addScreen(confirmationForm, 'ConfirmationCode', {
        text: 'Create account',
        action: () => props.actions.confirmRegistration(props.signupState),
        disabled: !confirmationForm.isValid()
    });

    return superForm;
}

function getLoginInfoForm(signupState: SignupState) {
    let loginInfoSection = new Section([
        new TextFieldElement('emailAddress', signupState, 'Email Address', 'email'),
        new TextFieldElement('password', signupState, 'Password', 'password'),
        new TextFieldElement('confirmedPassword', signupState, 'Password', 'password'),
    ]);
    return new SingleForm([loginInfoSection], 'Choose your login credentials');
}

function getPersonalDetailsForm(signupState: SignupState) {
    let personalDetailsSection = new Section([
        new TextFieldElement('givenName', signupState, 'Given name', 'given-name'),
        new TextFieldElement('familyName', signupState, 'Family name', 'family-name'),
        new TextFieldElement('familyName', signupState, 'Family name', 'family-name'),
        new TextFieldElement('addressLine1', signupState, 'Address line 1', 'address-line-1'),
        new TextFieldElement('addressLine2', signupState, 'Address line 2', 'address-line-2'),
        new TextFieldElement('postCode', signupState, 'Post code', 'postal-code'),
        new TextFieldElement('city', signupState, 'City or town', 'city'),
        new ReadonlyFieldElement('country', 'Country', 'country'),
    ]);
    return new SingleForm([personalDetailsSection], 'Enter your personal details');
}

function getConfirmationForm(signupState: SignupState) {
    let confirmationSection = new Section([
        new TextFieldElement('confirmationCode', signupState, 'Confirmation code')
    ]);
    return new SingleForm([confirmationSection], `Enter the confirmation code that was emailed to ${signupState.emailAddress}.`);
}
