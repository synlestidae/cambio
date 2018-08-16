import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';
import {SingleForm} from './form/single_form';
import {Section} from './form/section';
import {FieldElement} from './form/field_element';
import {OptionFieldElement} from './form/option_field_element';
import {CryptoAccount} from './domain/crypto_account';
import {TextFieldElement} from './form/text_field_element';
import {CurrencyFieldElement} from './form/currency_field_element';
import {ReadonlyFieldElement} from './form/readonly_field_element';
import {FormComponent} from './form_component';
import {LoadingState} from './flux/state/loading_state';
import {OrderRequest} from './domain/order_request';
import {SingleFormVisitor} from './form/single_form_visitor';
import {ReactSectionVisitor} from './form/react_section_visitor';

interface NewOrderComponentProps {
    newOrder: NewOrder,
    cryptoAccounts: CryptoAccount[],
    actions: ActionCreators
}

export function NewOrderComponent(props: NewOrderComponentProps): JSX.Element {
    let form = getForm(props);
    let loadingState = new LoadingState();

    if (props.newOrder.orderState === 'Submitting') {
        loadingState.startLoading();
    } else if (props.newOrder.orderState === 'Failed') {
        loadingState.name = 'Error';
        loadingState.message = 'There was an error submitting your order.';
    }
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

    let visitor = new SingleFormVisitor(() => props.actions.setOrderRequest(props.newOrder.order), new ReactSectionVisitor());
    form.accept(visitor);

    return <div className="order-modal">
        {visitor.render()}
    </div>;
}

function getForm(props: NewOrderComponentProps) {
    let order = props.newOrder.order;
    let fields: FieldElement[] = [];
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
    fields.push(new OptionFieldElement(
        'address', 
        order, 
        `You promise to make a transaction ${order.isBuy? 'from': 'to'}`, 
        props.cryptoAccounts.map((account: CryptoAccount) => ({ 
            label: `${account.name} (${account.address.substring(0, 8)}...)`, 
            value: account.address
        }))
    ));
    let price = order.getPrice();
    let formattedPrice = isNaN(price) || !isFinite(price)? '--' : price.toFixed(4);
    let priceField = new ReadonlyFieldElement(formattedPrice, 'ETH price (4 dp)');
    let section = new Section(fields.concat([priceField]));
    return new SingleForm([section], 
        () => props.actions.confirmNewOrder(order), 
        'Place a new order');
}
