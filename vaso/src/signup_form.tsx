import * as React from "react";
import {CalendarDate} from './domain/calendar_date';
import {ActionCreators} from './flux/action_creators';
import {SignupState} from './flux/state/signup_state';

export type FormState = 'EmailPassword' | 'PersonalDetails' | 'ProfileInfo';

interface PartialSignupFormProps {
    actions: ActionCreators;
}

interface FormElem {
    label: string;
    field: string;
    validate(val: string): string|null;
    form_type?: 'option';
}

class OptionElem implements FormElem {
    public options: string[];
    public label: string;
    public field: string;
    public readonly form_type = 'option';

    constructor(field: string, label: string, options: string[], msg: string) {
        this.field = field;
        this.label = label;
        this.options = options;
    }

    public validate(value: string): string|null {
        if (this.options.indexOf(value) >= 0) {
            return null;
        }
    }
}

export function SignupForm(props: SignupState & PartialSignupFormProps) {
    let elems: FormElem[] = [];
    const nonEmpty = function(msg: string) {
        return (val: string) => Boolean(val)? null : msg;
    };
    if (props.form_state == 'LoginInfo') {
        elems = [
            {label: 'Email address', field: 'email_address', validate: nonEmpty('Enter your email address.')},
            {label: 'Password', field: 'password', validate: nonEmpty('Enter a password of at least 8 characters.')},
            {label: 'Confirm password', field: 'password_confirm', validate: nonEmpty('Enter the password you typed above.')},
        ];
    } else if (props.form_state == 'PersonalInfo') {
        elems = [
            {label: 'First name(s)', field: 'first_names', validate: nonEmpty('Enter your given names (including middle names).')},
            {label: 'Family name', field: 'family_names', validate: nonEmpty('Enter your last or family name.')},
            {label: 'Address line 1', field: 'address_line_1', validate: nonEmpty('Enter the top line you would put on a letter.')},
            {label: 'Address line 2', field: 'address_line_2', validate: () => null},
            {label: 'Post code', field: 'post_code', validate: nonEmpty('Enter your address\' four-digit NZ post code.')},
            {label: 'City or town', field: 'city', validate: nonEmpty('Enter the place name of this address')},
            {label: 'Country', field: 'city', validate: () => null},
        ];
    } else if (props.form_state == 'Identification') {
        elems = [
            new OptionElem('id_type', 'Type of NZ identification', ['Passport_NZ', 'DriversLicence_NZ'], 'Please choose a type of ID'),
            {label: 'ID Number', field: 'id_number', validate: nonEmpty('Enter the ID number')}
        ];
    }
    return <form className="form-signin">
        {makeForm(elems, props.actions, props)};
    </form>;
}

function makeForm(elems: FormElem[], actions: ActionCreators, props: SignupState): JSX.Element[] {
    let jsxElements = [];
    for (let e of elems) {
        jsxElements.push(<div className="form-row">{getFormElement(e, actions, props)}</div>);
    }
    return jsxElements;
}

function getFormElement(elem: FormElem, actions: ActionCreators, value: any): JSX.Element {
    if (elem.form_type === 'option') {
        return (<div className="form-row">
              <label className="sr-only">{elem.label}</label>
              <input type="option" 
                id={'input_' + elem.field} 
                className="form-control" 
                value={value[elem.field]} 
                placeholder={elem.label}
                onChange={(e: any) => actions.setSignupFormValue(elem.field, e.target.value)}>
              </input>
            </div>);
    } else {
    }
}
