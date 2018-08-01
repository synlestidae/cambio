import * as React from "react";
import {Order} from './domain/order';
import {ActionCreators} from './flux/action_creators';
import {BoardPage} from './flux/state/board_page';
import {TableComponent, Column, OperationColumn, FieldColumn} from './table_component';
import {NewOrderComponent} from './new_order_component';
import {getUniqueID} from './flux/state/new_order';

interface BoardPageComponentProps {
    actions: ActionCreators,
    page: BoardPage
}

function getColumns(props: BoardPageComponentProps) {
    let actions = props.actions;
    let headers: Column<Order>[] = [];

    let tradeType = new FieldColumn<Order>('Trade type', 'isBuy', (o: Order) => o.isBuy? 'BUY' : 'SELL');
    let priceHeader = new FieldColumn<Order>('Ether unit price', 'price', (o: Order) => o.formatPrice() || '--');
    let expiryHeader = new FieldColumn<Order>('Expiry', 'expiresAt', (o: Order) => formatMinutes(o.expiresAt, new Date()));
    let statusHeader = new FieldColumn<Order>('Status', 'status', (o: Order) => getStatus(o));
    let operationHeader = new OperationColumn<Order>('Buy', 'buy', (o: Order) => {});

    headers.push(tradeType);
    headers.push(priceHeader);
    headers.push(expiryHeader);
    headers.push(statusHeader);
    headers.push(operationHeader);

    return headers;
}

function formatMinutes(date1: Date, date2: Date): string {
    let diffMilliseconds: number = date1.getTime() - date2.getTime();
    if (diffMilliseconds < 60000) {
        return 'Less than a minute';
    }
    return `${(diffMilliseconds / (1000 * 60)).toFixed(0)} minutes`;
}

function getStatus(o: Order) {
    if (o.expiresAt < new Date()) {
        return 'Expired';
    }
    return o.orderStatus;
}

export function BoardPageComponent(props: BoardPageComponentProps) {
    let orders = props.page.active_orders;
    let columns = getColumns(props);
    let sortCB = (field: string) => props.actions.sortOrders(field);
    orders = sortRows(orders, props.page.sortField);
    let newOrder = props.page.newOrder;
    let placeOrderModal; 
    if (newOrder) {
        placeOrderModal = <div className="modal-container">
          <NewOrderComponent newOrder={newOrder} actions={props.actions}>
          </NewOrderComponent>
        </div>; 
    } else {
        placeOrderModal = null;
    }
    const emptyMessage = 'No orders to show.';
    return <div>
        <div>
          <NewOrderButton onClick={(isBuy: boolean) => props.actions.newOrder(isBuy)} disabled={Boolean(newOrder)}></NewOrderButton>
          {placeOrderModal}
        </div>
        <TableComponent columns={columns} rows={orders} sortCB={sortCB} emptyMessage={emptyMessage}>
        </TableComponent>
    </div>;
}

function NewOrderButton(props: {onClick: (isBuy: boolean) => void, disabled: boolean}) {
    return <div>
      <button className="btn btn-primary" type="submit" onClick={() => props.onClick(true)} disabled={props.disabled}>
        Buy ETH
      </button>
      <button className="btn btn-primary" type="submit" onClick={() => props.onClick(false)} disabled={props.disabled}>
        Sell ETH
      </button>
    </div>;
}

function sortRows(orders: Order[], field: string): Order[]{
    orders = orders.filter(() => true);
    return orders.sort(function(o1: Order, o2: Order) {
        if (field === null || field === undefined) {
            return 0;
        }
        if (field === 'price') {
            return o1.getEthPrice() - o2.getEthPrice();
        }
        let val1 = (o1 as any)[field];
        let val2 = (o2 as any)[field];
        if (typeof val1 === 'string') {
            return val1.localeCompare(val2);
        }
        return val1 - val2;
    });
}
