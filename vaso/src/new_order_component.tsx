import * as React from "react";
import {NewOrder} from './flux/state/new_order';
import {CurrencyCode} from './domain/currency_code'; 
import {CurrencyDenom} from './domain/currency_denom'; 
import {ActionCreators} from './flux/action_creators';

interface NewOrderComponentProps {
    newOrder: NewOrder|null,
    actions: ActionCreators
}

export function NewOrderComponent(props: NewOrderComponentProps) {
    const onSelectBuy = (c: CurrencyCode) => props.actions.setNewOrderBuyCurrency(c);
    const onSelectSell = (c: CurrencyCode) => props.actions.setNewOrderSellCurrency(c);
    const onChangeBuyUnits = (v: number) => props.actions.setNewOrderBuyUnits(v);
    const onChangeSellUnits = (v: number) => props.actions.setNewOrderSellUnits(v);

    let order = props.newOrder.order;
    return <div className="flex-horizontal">
        <div className="flex-vertical">
            <label>You want to buy</label>
            <CurrencyDropdown selected={order.buy_asset_type} onSelect={onSelectBuy}>
            </CurrencyDropdown>
        </div>
        <div className="flex-vertical">
            <label>Units to buy</label>
            <CurrencyInput 
              currencyCode={order.buy_asset_type} 
              currencyDenom={order.buy_asset_denom} 
              value={order.buy_asset_units} 
              onChange={onChangeBuyUnits}>
            </CurrencyInput>
        </div>
        <div className="flex-vertical">
            <label>You want to sell</label>
            <CurrencyDropdown selected={order.sell_asset_type} onSelect={onSelectSell}>
            </CurrencyDropdown>
        </div>
        <div className="flex-vertical">
            <label>Units to sell</label>
            <CurrencyInput 
              currencyCode={order.sell_asset_type} 
              currencyDenom={order.sell_asset_denom} 
              value={order.sell_asset_units} 
              onChange={onChangeSellUnits}>
            </CurrencyInput>
        </div>
        <div className="flex-vertical">
            <label>Confirm</label>
            <input type="text" value={props.newOrder.unique_id} disabled></input>
            <input type="text"></input>
        </div>
    </div>
}

interface CurrencyDropdownProps {
    selected: CurrencyCode|null,
    onSelect: (currency: CurrencyCode) => void 
}

function CurrencyDropdown(props: CurrencyDropdownProps) {
    let options = getCurrencies()
        .map((c: CurrencyCode, i: number) => {
            return <option value={c} key={i}>{c}</option>;
        });
    return <select value={props.selected} onChange={(e: any) => props.onSelect(e.target.value as CurrencyCode)}>
        {options}  
    </select>;
}

function getCurrencies(): CurrencyCode[] {
    return [
        'NZD',
        'ETH'
    ];
}

interface CurrencyInputProps {
    currencyCode: CurrencyCode,
    currencyDenom: CurrencyDenom,
    value: number,
    onChange: (n: number) => void,
}

function CurrencyInput(props: CurrencyInputProps) {
    return <div>
        <input type="number" value={props.value} onChange={(e: any) => props.onChange(e.target.value as number)}>
        </input>
        <span> 
          <i>{formatCurrency(props.value, props.currencyCode, props.currencyDenom)}</i>
        </span>
    </div>;
}

function formatCurrency(value: number, code: CurrencyCode, denom: CurrencyDenom) {
    if (code === 'NZD') {
        if (denom === 'Cent') {
            value = value / 100;
        }
        return `$${value.toFixed(2)}`;
    } else if (code === 'ETH') {
        if (denom === 'Szabo') {
            value = value / 1000000;
        } else if (denom === 'Wei') {
            throw new Error('Cannot format Wei as Eth'); 
        }
        return `ETH ${value}`;
    }
}
