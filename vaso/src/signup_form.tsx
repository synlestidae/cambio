import * as React from "react";
import {CalendarDate} from './domain/calendar_date';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';

export type FormState = 'EmailPassword' | 'PersonalDetails' | 'ProfileInfo';

interface PartialSignupFormProps {
    actions: ActionCreators;
}

export interface FormElem {
    label: string;
    field: string;
    validate(val: string): string|null;
    formType?: 'option' | 'password' | 'text';
    name?: string;
    value: string;
    disabled?: boolean;
}

export class OptionElem implements FormElem {
    public options: string[];
    public label: string;
    public field: string;
    public value: string;
    public readonly formType = 'option';

    constructor(field: string, label: string, options: string[], msg: string, value: string) {
        this.field = field;
        this.label = label;
        this.options = options;
        this.value = value;
    }

    public validate(value: string): string|null {
        if (this.options.indexOf(value) >= 0) {
            return null;
        }
    }
}

export function buildForm(props: SignupState & PartialSignupFormProps): FormElem[] {
    let elems: FormElem[] = [];

    const nonEmpty = function(msg: string) {
        return (val: string) => Boolean(val)? null : msg;
    };
    if (props.form_state == 'LoginInfo') {
        elems = [
            {
                label: 'Email address', 
                field: 'email_address', 
                validate: nonEmpty('Enter your email address.'),
                name: 'email',
                value: props.loginInfo.email_address
            },
            {
                label: 'Password', 
                field: 'password', 
                validate: (val: string) => val.length >= 8? null : 'Enter a password of at least 8 characters.', 
                formType: 'password',
                name: 'password',
                value: props.loginInfo.password
            },
            {
                label: 'Confirm password', 
                field: 'passwordConfirm', 
                validate: () => props.loginInfo.password === props.loginInfo.passwordConfirm? null : 'Passwords must match.',
                formType: 'password',
                name: 'password',
                value: props.loginInfo.passwordConfirm
            }
        ];
    } else if (props.form_state == 'PersonalInfo') {
        elems = [
            {
                label: 'First name(s)',
                field: 'first_names',
                validate: nonEmpty('Enter your given names (including middle names).'),
                name: "given-name",
                value: props.info.first_names
            },
            {
                label: 'Family name',
                field: 'family_name',
                validate: nonEmpty('Enter your last or family name.'),
                name: "family-name",
                value: props.info.family_name
            },
            {
                label: 'Address line 1',
                field: 'address_line_1',
                validate: nonEmpty('Enter the top line you would put on a letter.'),
                name: 'address-line1',
                value: props.info.address_line_1
            },
            {
                label: 'Address line 2',
                field: 'address_line_2',
                validate: () => null,
                name: 'address-line2',
                value: props.info.address_line_2
            },
            {
                label: 'Post code',
                field: 'post_code',
                validate: nonEmpty('Enter your address\' four-digit NZ post code.'),
                name: 'postal-code',
                value: props.info.post_code
            },
            {
                label: 'City or town',
                field: 'city', 
                validate: nonEmpty('Enter the place name of this address'),
                name: 'city',
                value: props.info.city
            },
            {
                label: 'Country',
                field: 'country',
                validate: () => null,
                value: 'New Zealand',
                disabled: true
            },
        ];
    } else if (props.form_state == 'Identification') {
        elems = [
            new OptionElem(
                'id_type', 
                'Type of NZ identification', 
                ['Passport_NZ', 'DriversLicence_NZ'],
                'Please choose a type of ID',
                props.identification.id_type
            ),
            {
                label: 'ID Number', 
                field: 'id_number', 
                validate: nonEmpty('Enter the ID number'),
                value: props.identification.id_number
            }
        ];
    }
    return elems;
}

export function SignupForm(elems: FormElem[], actions: ActionCreators, props: SignupState): JSX.Element[] {
    return makeForm(elems, actions, props);
}

function makeForm(elems: FormElem[], actions: ActionCreators, props: SignupState): JSX.Element[] {
    let jsxElements = [];
    let i = 0;
    for (let e of elems) {
        jsxElements.push(getFormElement(e, actions, props.dirtyFields.has(e.field)));
        i++;
    }
    return jsxElements;
}

function getFormElement(elem: FormElem, actions: ActionCreators, isDirty: boolean): JSX.Element {
    let inputValue = elem.value;
    const handleBlur = function() {
        actions.clearDirtyValue(elem.field);
        if (elem.validate(inputValue) !== null) {
            actions.addDirtyValue(elem.field);
            console.log('Blur!', elem, inputValue);
        }
    };
    const handleFocus = function() {
        console.log('Folks!', elem.field);
        actions.clearDirtyValue(elem.field);
    };
    console.log('Dirty?', elem.field, inputValue, isDirty);
    let validation: string|null = isDirty && elem.validate(inputValue);
    if (elem.formType === 'option') {
        return (<div className="form-row">
              <label className="sr-only">{elem.label}</label>
              <input type="option" 
                id={'input_' + elem.field} 
                className="form-control" 
                value={inputValue} 
                disabled={elem.disabled}
                placeholder={elem.label}
                onChange={(e: any) => actions.setSignupFormValue(elem.field, e.target.value)}>
              </input>
            </div>);
    } else {
        return (<div className="form-row">
              <label className="sr-only">{elem.label}</label>
              <input 
                type={elem.formType === 'password'? 'password' : 'text'}
                id={'signup_' + elem.field} 
                className="form-control" 
                value={inputValue} 
                placeholder={elem.label}
                autoComplete={elem.name? 'on' : 'off'}
                disabled={elem.disabled}
                name={elem.name || elem.field}
                onBlur={handleBlur}
                onFocus={handleFocus}
                onChange={(e: any) => actions.setSignupFormValue(elem.field, e.target.value)}>
              </input>
              <ValidationMessage msg={validation}></ValidationMessage>
            </div>);
    }
}

function ValidationMessage(props: {msg: string|null}) {
    return <div className="error-text">
        <em className="smaller-font">
            {props.msg}
        </em>
    </div>;
}
