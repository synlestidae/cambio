import {NewOrder, OrderState} from './flux/state/new_order';
import {ActionCreators} from './flux/action_creators';

import * as React from "react";

export interface ConfirmOrderComponent {
    actions: ActionCreators,
    newOrder: NewOrder
}

function pad(n: string) {
    if (n.length < 2) {
        return '0' + n;
    }
    return n;
}

export function ConfirmOrderComponent(props: ConfirmOrderComponent) {
    let order = props.newOrder.order;
    let actions = props.actions;

    const cancelOrder = () => actions.cancelNewOrder();
    const editOrder = () => actions.editNewOrder();
    const sendOrder = () => actions.confirmNewOrder(order);
    const onChangeConfirm = (v: string) => props.actions.setNewOrderUniqueId(v);

    let expiry = order.expiry;
    let minutes = pad(expiry.getMinutes().toString());
    let hours = pad(expiry.getHours().toString());
    let dateStr = expiry.toDateString();
    let date: string = `${dateStr} ${hours}:${minutes}`;

    let state = props.newOrder.orderState;

    return <div>
        <div>
          <span>You buy: </span> 
          <span>{order.buy_asset_units} {order.buy_asset_type} {order.buy_asset_denom}</span> 
        </div>
        <div>
          <span>You sell: </span>
          <span>{order.sell_asset_units} {order.sell_asset_type} {order.sell_asset_denom}</span> 
        </div>
        <div>
          <span>Ethereum price: </span>
          <span>{order.formatPrice()}</span>
        </div>
        <div>
          <span>Expires at: </span> 
          <span>{date} ({order.formatExpiryMinutes()})</span>
        </div>
        <div>
            <div>Confirm your order</div>
            <div>
                <input type="text" className="form-control" value={props.newOrder.unique_id} disabled>
                </input>
            </div>
            <div> 
                <input type="text" 
                  className="form-control" 
                  onChange={(e: any) => onChangeConfirm(e.target.value as string)} 
                  value={props.newOrder.order.unique_id} 
                  placeholder="Copy the text above">
                </input>
            </div>
            <p>This will be your order's unique ID. Copy it exactly. This helps to prevent data entry errors.</p>
        </div>
        <OrderButtons editOrder={editOrder} cancelOrder={cancelOrder} sendOrder={sendOrder} state={state}>
        </OrderButtons>
        <OrderText state={state}></OrderText>
    </div>;
}

interface OrderButtonsInterface {
    editOrder: () => void,
    cancelOrder: () => void,
    sendOrder: () => void,
    state: OrderState
}

function OrderButtons(props: OrderButtonsInterface) {
    let state = props.state;
    if (state === 'Success' || state === 'Failed') {
        return <button className="btn btn-primary" onClick={props.cancelOrder}>
          Close
        </button>
    }
    return <div>
        <button className="btn btn-primary" onClick={props.editOrder} disabled={state !== 'ReadyToConfirm'}>
          Edit
        </button>
        <button className="btn btn-primary" onClick={props.cancelOrder} disabled={state  == 'Submitting'}>
          Cancel
        </button>
        <button className="btn btn-link" onClick={props.sendOrder} disabled={state != 'ReadyToConfirm'}>
          Confirm and send
        </button>
    </div>
}

function OrderText(props: {state: OrderState}) {
    if (props.state === 'Failed') {
        // TODO user doesn't know what their order was
        return <div>An error occurred while submitting. But order may have been accepted. Check the board for your order and try again. </div>;
    }
    if (props.state === 'Success') {
        return <div>Your order was accepted!</div>;
    }
    return null;
}
