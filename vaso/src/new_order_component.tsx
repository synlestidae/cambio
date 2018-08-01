import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';
import {SingleForm} from './form/single_form';
import {Section} from './form/section';
import {FieldElement} from './form/field_element';
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
    let fields: FieldElement[];
    if (order.isBuy) {
        fields = [
            new CurrencyFieldElement('ether', order, 'ETH to buy'),
            new CurrencyFieldElement('dollars', order, 'NZD to sell')
        ];
    } else {
        fields = [
            new CurrencyFieldElement('dollars', order, 'NZD to buy'),
            new CurrencyFieldElement('ether', order, 'ETH to sell')
        ];
    }
    let price = order.getPrice();
    let formattedPrice = isNaN(price) || !isFinite(price)? '--' : price.toFixed(4);
    let priceField = new ReadonlyFieldElement(formattedPrice, 'ETH price (4 dp)');
    let section = new Section(fields.concat([priceField]));
    let form = new SingleForm([section], 'Place a new order');
    if (props.newOrder.orderState === 'Submitting') {
        form.state.startLoading();
    } else if (props.newOrder.orderState === 'Failed') {
        form.state.name = 'Error';
        form.state.message = 'There was an error submitting your order.';
    }
    form.onChange = function() {
        props.actions.setOrderRequest(order);
    };
    if (props.newOrder.orderState === 'Submitting') {
        return <div className="order-modal">Submitting your order now...</div>
    }
    if (props.newOrder.orderState.toString() === 'Success') {
        return <div className="order-modal">
            <div>Your order submitted succcessfully!</div>
            <div>
              <button className="btn btn-primary" onClick={() => props.actions.clearOrder()}>
                Close
              </button>
            </div>
        </div>;
    }
    return <div className="order-modal">
        <FormComponent 
          form={form} 
          onCancel={() => props.actions.cancelNewOrder()}
          onSubmit={() => {
              props.actions.confirmNewOrder(order);
              return false;
            }
          }>
        </FormComponent>
    </div>;
}
