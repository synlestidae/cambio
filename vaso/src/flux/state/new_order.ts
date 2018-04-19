import {OrderRequest} from '../../domain/order_request';

export class NewOrder {
    public unique_id: string = getUniqueID(10);
    public orderState: OrderState = 'Initial';
    public order: OrderRequest;

    constructor() {
        let defaultExpiry = new Date(); 
        defaultExpiry.setMinutes(defaultExpiry.getMinutes() + 15);
        this.order = new OrderRequest('', defaultExpiry, 'Active', 'NZD', 'Cent', 0, 'ETH', 'Szabo', 0);
    }
}

export type OrderState = 'Initial' | 'ReadyToSubmit' | 'Failed' | 'Submitted';

function getUniqueID(length: number) {
    const POSSIBLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";
    let uniqueID = '';
    for (let i = 0; i < length; i++) {
        let rdx = Math.floor(POSSIBLE.length * Math.random());
        let c = POSSIBLE.charAt(rdx);
        uniqueID += c;
    }
    return uniqueID;
}
