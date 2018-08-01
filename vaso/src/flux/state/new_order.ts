import {OrderRequest} from '../../domain/order_request';

export class NewOrder {
    public unique_id: string = getUniqueID(10);
    public orderState: OrderState = 'Initial';
    public order: OrderRequest;
    public showValidation = false;

    constructor(isBuy: boolean) {
        this.order = new OrderRequest();
        this.order.isBuy = isBuy;
        this.order.uniqueId = getUniqueID(12);
    }
}

export type OrderState = 'Initial' | 'ReadyToConfirm' | 'Submitting' | 'Failed' | 'Success';

export function getUniqueID(length: number) {
    const POSSIBLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";
    let uniqueID = '';
    for (let i = 0; i < length; i++) {
        let rdx = Math.floor(POSSIBLE.length * Math.random());
        let c = POSSIBLE.charAt(rdx);
        uniqueID += c;
    }
    return uniqueID;
}
