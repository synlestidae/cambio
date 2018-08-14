import * as React from "react";
import {Order} from './domain/order';
import {ActionCreators} from './flux/action_creators';
import {BoardPage} from './flux/state/board_page';
import {Table} from './table/table';
import {Column} from './table/column';
import {FieldColumn} from './table/field_column';
import {TableVisitor} from './table/table_visitor';
import {NewOrderComponent} from './new_order_component';
import {getUniqueID} from './flux/state/new_order';
import {ReactTableVisitor} from './table/react_table_visitor';

interface BoardPageComponentProps {
    actions: ActionCreators,
    page: BoardPage
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

    let columns = getColumns(props);
    let table = new Table(getColumns(props), orders);
    let visitor = new ReactTableVisitor();
    table.accept(visitor)

    return <div>
        <div>
          <NewOrderButton onClick={(isBuy: boolean) => props.actions.newOrder(isBuy)} disabled={Boolean(newOrder)}></NewOrderButton>
          {placeOrderModal}
        </div>
        {visitor.render()}
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

function getColumns(props: BoardPageComponentProps) {
    let actions = props.actions;
    let headers: Column<Order>[] = [];

    let tradeType = new FieldColumn<Order>('Trade type', 'isBuy', (o: Order) => o.isBuy? 'BUY' : 'SELL');
    let priceHeader = new FieldColumn<Order>('Ether unit price', 'price', (o: Order) => o.formatPrice() || '--');
    let expiryHeader = new FieldColumn<Order>('Expiry', 'expiresAt', (o: Order) => formatMinutes(o.expiresAt, new Date()));
    let statusHeader = new FieldColumn<Order>('Status', 'status', (o: Order) => getStatus(o));
    let operationHeader = new FieldColumn<Order>('Buy', 'buy', (o: Order) => 'Buy');

    headers.push(tradeType);
    headers.push(priceHeader);
    headers.push(expiryHeader);
    headers.push(statusHeader);
    headers.push(operationHeader);

    return headers;
}
