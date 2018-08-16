import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';

export class OrderRequest {
    ether: number = 0;
    dollars: number = 0;
    uniqueId: string = '';
    address: string = '';
    isBuy: boolean = true;
    minutesActive = 15;

    public getPrice(): number {
        return this.dollars / this.ether;
    }
}
