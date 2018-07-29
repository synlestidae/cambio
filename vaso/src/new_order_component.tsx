import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';

interface NewOrderComponentProps {
    newOrder: NewOrder,
    actions: ActionCreators
}

export function NewOrderComponent(props: NewOrderComponentProps) {
    const onChangeBuyUnits = (v: any) => props.actions.setNewOrderBuyUnits(v.target.value);
    const onChangeSellUnits = (v: any) => props.actions.setNewOrderSellUnits(v.target.value);
    const onSubmit = () => props.actions.startNewOrderConfirm();
    const onCancel = () => props.actions.cancelNewOrder();
    return <div className="modal order-modal">
        <div className="form-group">
          <label>ETH to buy</label>
          <input className="form-control" type="number" step="0.001" min="0" max="1000" onChange={onChangeBuyUnits}>
          </input>
        </div>
        <div className="form-group">
          <label>NZD to sell</label>
          <input max="5000" min="0" step="0.01" type="number" className="form-control" onChange={onChangeSellUnits}>
          </input>
        </div>
        <div className="form-group">
          <label>ETH Price</label>
          <input type="text" disabled className="form-control" value="3863.8462">
          </input>
        </div>
        <div className="form-group">
          <button className="non-touching-button btn btn-primary" onClick={onSubmit}>Cancel</button>
          <button className="non-touching-button btn btn-primary" onClick={onCancel}>Submit order</button>
        </div>
      </div>;
}
