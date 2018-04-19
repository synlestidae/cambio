import * as React from "react";
import {UserOrder} from './domain/user_order';
import {ActionCreators} from './flux/action_creators';
import {BoardPage} from './flux/state/board_page';
import {TableComponent, Column, FieldColumn} from './table_component';
import {NewOrderComponent} from './new_order_component';
import {ConfirmOrderComponent} from './confirm_order_component';

interface BoardPageComponentProps {
    actions: ActionCreators,
    page: BoardPage
}

function getColumns() {
    let headers: FieldColumn<UserOrder>[] = [];

    let sellHeader = new FieldColumn<UserOrder>('Wants to sell', 'sell_asset_type', (o: UserOrder) => o.sell_asset_type);
    let buyHeader = new FieldColumn<UserOrder>('Wants to buy', 'buy_asset_type', (o: UserOrder) => o.buy_asset_type);
    let priceHeader = new FieldColumn<UserOrder>('Ether unit price', 'price', (o: UserOrder) => o.formatPrice() || '--');
    let expiryHeader = new FieldColumn<UserOrder>('Expiry', 'expiry', (o: UserOrder) => o.formatExpiryMinutes());
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
    let newOrderComponent; //= newOrder? 
    if (!newOrder) {
        newOrderComponent = <NewOrderButton onClick={() => props.actions.newOrder()}>
        </NewOrderButton>;
    } else if (newOrder.orderState === 'Initial') {
        newOrderComponent = <NewOrderComponent newOrder={newOrder} actions={props.actions}>
        </NewOrderComponent>; 
    } else {
        newOrderComponent = <ConfirmOrderComponent actions={props.actions} newOrder={newOrder}>
            </ConfirmOrderComponent>;
    }
    return <div>
        <div>
          {newOrderComponent}
        </div>
        <TableComponent columns={columns} rows={orders} sortCB={sortCB}>
        </TableComponent>
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
            return o1.getEthPrice() - o2.getEthPrice();
        }
        let val1 = (o1 as any);
        let val2 = (o2 as any);
        if (typeof val1 === 'string') {
            return val1.localeCompare(val2);
        }
        return val1 - val2;
    });
}
