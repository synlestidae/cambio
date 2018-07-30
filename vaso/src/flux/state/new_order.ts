import {OrderRequest} from '../../domain/order_request';

export class NewOrder {
    public unique_id: string = getUniqueID(10);
    public orderState: OrderState = 'Initial';
    public order: OrderRequest;
    public showValidation = false;
    public readonly isBuy: boolean;

    constructor(isBuy: boolean) {
        this.isBuy = isBuy;
        let defaultExpiry = new Date(); 
        defaultExpiry.setMinutes(defaultExpiry.getMinutes() + 15);
        if (isBuy) {
            this.order = new OrderRequest('', defaultExpiry, 'Active', 'NZD', 'Cent', 0, 'ETH', 'Szabo', 0);
        } else {
            this.order = new OrderRequest('', defaultExpiry, 'Active', 'ETH', 'Szabo', 0, 'NZD', 'Cent', 0);
        }
        this.order.unique_id = getUniqueID(12);
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
