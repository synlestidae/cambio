import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';
import {SingleForm} from './form/single_form';
import {Section} from './form/section';
import {TextFieldElement} from './form/text_field_element';
import {CurrencyFieldElement} from './form/currency_field_element';
import {ReadonlyFieldElement} from './form/readonly_field_element';
import {FormComponent} from './form_component';

interface NewOrderComponentProps {
    newOrder: NewOrder,
    actions: ActionCreators
}

export function NewOrderComponent(props: NewOrderComponentProps): JSX.Element {
    let order = props.newOrder.order;
    let ethField = new CurrencyFieldElement('buy_asset_units', order, 'ETH to buy');
    let currencyField = new CurrencyFieldElement('sell_asset_units', order, 'NZD to sell');
    let priceField = new ReadonlyFieldElement(
        (order.buy_asset_units / order.sell_asset_units).toFixed(4), 
        'Price per ETH (4 dp)');
    ethField.decimalPlaces = 4;
    currencyField.decimalPlaces = 2;
    let section = new Section([
        ethField, 
        currencyField,
        priceField
    ]);
    let form = new SingleForm([section], 'Place a new order', function(){}, function(){});
    form.onChange = function() {
        props.actions.setOrderRequest(order);
    };
    return <div className="order-modal">
        <FormComponent 
          form={form} 
          onCancel={() => props.actions.cancelNewOrder()}
          onSubmit={() => props.actions.confirmNewOrder(order)}>
        </FormComponent>
    </div>;
}
