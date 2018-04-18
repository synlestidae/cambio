import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';
import {Order} from './order';

export class UserOrder extends Order {
    public id: string;
    public expiry: Date
    public status: string;

    constructor(
        id: string,
        expiry: Date,
        status: string,
        sell_asset_type: CurrencyCode,
        sell_asset_denom: CurrencyDenom,
        sell_asset_units: number,
        buy_asset_type: CurrencyCode,
        buy_asset_denom: CurrencyDenom,
        buy_asset_units: number) {
        super(sell_asset_type, 
            sell_asset_denom, 
            sell_asset_units, 
            buy_asset_type, 
            buy_asset_denom, 
            buy_asset_units);
        this.id = id;
        this.expiry = expiry;
        this.status = status;
    }
}
