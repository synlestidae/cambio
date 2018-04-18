import {Order} from '../../domain/Order';

export class NewOrder {
    public order: Order = new Order('NZD', 'Cent', 0, 'ETH', 'Szabo', 0);
    public unique_id: string = getUniqueID(10);
}

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
