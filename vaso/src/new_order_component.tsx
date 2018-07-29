import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';
import {SingleForm} from './form/single_form';
import {Section} from './form/section';
import {TextFieldElement} from './form/text_field_element';
import {FormComponent} from './form_component';

interface NewOrderComponentProps {
    newOrder: NewOrder,
    actions: ActionCreators
}

export function NewOrderComponent(props: NewOrderComponentProps): JSX.Element {
    let section = new Section([
        new TextFieldElement('sell_asset_units', props.newOrder.order, 'ETH to buy'), 
        new TextFieldElement('buy_asset_units', props.newOrder.order, 'NZD to sell'), 
    ]);
    let form = new SingleForm([section], 'Place a new order', function(){}, function(){});
    console.log('formy', form);
    return <div className="order-modal">
        <FormComponent form={form} onSubmit={() => {}} onCancel={() => {}}>
        </FormComponent>
    </div>;
}
