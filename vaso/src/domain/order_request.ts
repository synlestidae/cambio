import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';
import {Order} from './order';

export class OrderRequest extends Order {
    unique_id: string;
    sell_asset_type: CurrencyCode;
    sell_asset_denom: CurrencyDenom;
    sell_asset_units: number;
    buy_asset_type: CurrencyCode;
    buy_asset_denom: CurrencyDenom;
    buy_asset_units: number;
    expires_at: Date;
}
