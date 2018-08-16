import * as React from "react";
import {CalendarDate} from './domain/calendar_date';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';
import {Form} from './form/form';
import {Section} from './form/section';
import {SingleForm} from './form/single_form';
import {TextFieldElement} from './form/text_field_element';
import {PasswordFieldElement} from './form/password_field_element';
import {ReadonlyFieldElement} from './form/readonly_field_element';
import {RequiredTextFieldElement} from './form/required_text_field_element';
import {FieldElement} from './form/field_element';
import {SuperForm} from './form/super_form';
import {SignupStateName} from './flux/state/signup_state_name';

interface PartialSignupFormProps {
    actions: ActionCreators;
}

export function buildSignupForm(props: {signupState: SignupState} & PartialSignupFormProps): SuperForm<SignupStateName> {
    let elems: FieldElement[];
    let header: string;
    let signupState = props.signupState;

    let formState = props.signupState.formState;

    let loginInfoForm = getLoginInfoForm(props.signupState, 
        () => props.actions.submitSignup(signupState.emailAddress, signupState.password));
    let personalDetailsForm = getPersonalDetailsForm(props.signupState, 
        () => props.actions.setSignupStatePage('ConfirmationCode'));
    let confirmationForm = getConfirmationForm(props.signupState, 
        () => props.actions.sendRegistration(signupState));

    let superForm = new SuperForm(formState);
    superForm.onChange = () => props.actions.setSignupState(props.signupState);
    superForm.addScreen(loginInfoForm, 'LoginInfo',  {
        text: 'Add personal details',
        action: () => props.actions.sendRegistration(signupState),
    }, {
        text: 'Back',
        action: () => props.actions.loginMode(),
    });

    superForm.addScreen(personalDetailsForm, 'PersonalDetails', {
        text: 'Confirm email',
        action: () => props.actions.setSignupStatePage('ConfirmationCode'),
    }, {
        text: 'Back',
        action: () => props.actions.setSignupStatePage('LoginInfo'),
    });

    superForm.addScreen(confirmationForm, 'ConfirmationCode', {
        text: 'Create account',
        action: () => props.actions.confirmRegistration(props.signupState),
    }, {
        text: 'Back',
        action: () => props.actions.setSignupStatePage('PersonalDetails'),
    });

    superForm.loadingState = props.signupState.loadingState;
    loginInfoForm.loadingState = props.signupState.loadingState;
    personalDetailsForm.loadingState = superForm.loadingState;
    confirmationForm.loadingState = superForm.loadingState;

    return superForm;
}

function getLoginInfoForm(signupState: SignupState, onSubmit: () => void) {
    let loginInfoSection = new Section([
        new RequiredTextFieldElement('emailAddress', signupState, 'Email Address', 'email'),
        new PasswordFieldElement('password', signupState, 'Password', 'password'),
        new PasswordFieldElement('confirmedPassword', signupState, 'Confirm password', 'password'),
    ]);
    return new SingleForm([loginInfoSection], onSubmit, 'Choose your login credentials');
}

function getPersonalDetailsForm(signupState: SignupState, onSubmit: () => void) {
    let personalDetailsSection = new Section([
        new RequiredTextFieldElement('firstName', signupState, 'Given name', 'given-name'),
        new RequiredTextFieldElement('familyName', signupState, 'Family name', 'family-name'),
        new RequiredTextFieldElement('addressLine1', signupState, 'Address line 1', 'address-line-1'),
        new TextFieldElement('addressLine2', signupState, 'Address line 2', 'address-line-2'),
        new RequiredTextFieldElement('postCode', signupState, 'Post code', 'postal-code'),
        new RequiredTextFieldElement('city', signupState, 'City or town', 'city'),
        new ReadonlyFieldElement('New Zealand', 'Country', 'country'),
    ]);
    return new SingleForm([personalDetailsSection], onSubmit, 'Enter your personal details');
}

function getConfirmationForm(signupState: SignupState, onSubmit: () => void) {
    let confirmationSection = new Section([
        new TextFieldElement('confirmationCode', signupState, 'Confirmation code')
    ]);
    return new SingleForm([confirmationSection], onSubmit, `Enter the confirmation code that was emailed to ${signupState.emailAddress}.`);
}
