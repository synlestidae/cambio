import * as React from "react";
import {UserOrder} from './domain/user_order';
import {ActionCreators} from './flux/action_creators';
import {BoardPage} from './flux/state/board_page';
import {TableComponent, Column, FieldColumn} from './table_component';
import {NewOrderComponent} from './new_order_component';

interface BoardPageComponentProps {
    actions: ActionCreators,
    page: BoardPage
}

function getColumns() {
    let headers: FieldColumn<UserOrder>[] = [];

    let sellHeader = new FieldColumn<UserOrder>('Wants to sell', 'sell_asset_type', (o: UserOrder) => o.sell_asset_type);
    let buyHeader = new FieldColumn<UserOrder>('Wants to buy', 'buy_asset_type', (o: UserOrder) => o.buy_asset_type);
    let priceHeader = new FieldColumn<UserOrder>('Price', 'price', formatPrice);
    let expiryHeader = new FieldColumn<UserOrder>('Expiry', 'expiry', formatExpiry);
    let statusHeader = new FieldColumn<UserOrder>('Status', 'status', (o: UserOrder) => o.status);

    headers.push(sellHeader);
    headers.push(buyHeader);
    headers.push(priceHeader);
    headers.push(expiryHeader);
    headers.push(statusHeader);

    return headers;
}

export function BoardPageComponent(props: BoardPageComponentProps) {
    let orders = props.page.active_orders;
    let columns = getColumns();
    let sortCB = (field: string) => props.actions.sortOrders(field);
    orders = sortRows(orders, props.page.sortField);
    let newOrder = props.page.newOrder;
    let newOrderComponent = newOrder? 
        <NewOrderComponent newOrder={newOrder} actions={props.actions}>
        </NewOrderComponent> : 
        <NewOrderButton onClick={() => props.actions.newOrder()}>
        </NewOrderButton>;
    return <div>
        <TableComponent columns={columns} rows={orders} sortCB={sortCB}>
        </TableComponent>
        <div>
          {newOrderComponent}
        </div>
    </div>;
}

function NewOrderButton(props: {onClick: () => void}) {
    return <button className="btn btn-primary" type="submit" onClick={props.onClick}>
        Place order
    </button>;
}

function sortRows(orders: UserOrder[], field: string): UserOrder[]{
    orders = orders.filter(() => true);
    return orders.sort(function(o1: UserOrder, o2: UserOrder) {
        if (!field) {
            return 0;
        }
        if (field === 'price') {
            return getPrice(o1) - getPrice(o2);
        }
        let val1 = (o1 as any);
        let val2 = (o2 as any);
        if (typeof val1 === 'string') {
            return val1.localeCompare(val2);
        }
        return val1 - val2;
    });
}

function getPrice(order: UserOrder) {
    if (order.buy_asset_type === 'NZD') {
        return order.buy_asset_units / order.sell_asset_units;
    } else {
        return order.sell_asset_units / order.buy_asset_units;
    }
}

function formatPrice(order: UserOrder) {
    let price: number = getPrice(order);
    let priceWithDP = price.toFixed(4); 
    return `$${priceWithDP}`;
}

function formatExpiry(order: UserOrder): string {
    let date = order.expiry;
    let delta = date.getTime() - new Date().getTime();
    if (delta <= 0) {
        return '--';
    }
    let minutes = delta / (1000 * 60);
    if (minutes < 1) {
        return 'Less than a minute';
    }
    if (minutes < 2) {
        return '1 minute';
    }
    let minutesDP = minutes.toFixed(0);
    return `${minutesDP} minutes`;
}
