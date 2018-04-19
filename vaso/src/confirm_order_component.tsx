import {NewOrder} from './flux/state/new_order';
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
    const cancelOrder = () => {};
    const sendOrder = () => {};
    let expiry = order.expiry;
    let minutes = pad(expiry.getMinutes().toString());
    let hours = pad(expiry.getHours().toString());
    let dateStr = expiry.toDateString();
    let date: string = `${dateStr} ${hours}:${minutes}`;

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
          <span>{order.getEthPrice()}</span>
        </div>
        <div>
          <span>Expires at:</span> 
          <span>{date} ({order.formatExpiryMinutes()})</span>
        </div>
        <div>
            <button className="btn btn-primary" onClick={cancelOrder}>
              Edit
            </button>
            <button className="btn btn-primary" onClick={cancelOrder}>
              Cancel
            </button>
            <button className="btn btn-link" onClick={sendOrder}>
              Confirm and send
            </button>
        </div>
    </div>;
}
