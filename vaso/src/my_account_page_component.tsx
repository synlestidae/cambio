import * as React from "react";
import {MyAccount} from './flux/state/my_account';
import {ActionCreators} from './flux/action_creators';
import {TextFieldElement} from './form/text_field_element';
import {Section} from './form/section';
import {SingleForm} from './form/single_form';
import {PersonalDetails} from './domain/personal_details';
import {FormComponent} from './form_component';

interface MyAccountPageComponentProps {
    actions: ActionCreators,
    page: MyAccount
}

export function MyAccountPageComponent(props: MyAccountPageComponentProps) {
    if (props.page.loadingState.name === 'Loading') {
        return <div>Loading your details...</div>;
    }
    if (props.page.personalDetails === null) {
        return <div>Your details are currently unavailable.</div>;
    }
    let details: PersonalDetails = props.page.personalDetails;
    let elements = [
        new TextFieldElement('first_names', details, 'Given names'),
        new TextFieldElement('family_name', details, 'Family name'),
        new TextFieldElement('address_line_1', details, 'Address line 1'),
        new TextFieldElement('address_line_2', details, 'Address line 2'),
        new TextFieldElement('post_code', details, 'Post code'),
        new TextFieldElement('city', details, 'City'),
    ];
    let section = new Section(elements);
    let form = new SingleForm([section], 'Your details');
    form.onChange = function() {
        props.actions.setPersonalDetails(details);
    };
    return <FormComponent form={form} onSubmit={() => props.actions.updatePersonalDetails(details)}/>
}
