import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';
import {UserOrder} from './user_order';

export class OrderRequest extends UserOrder {
    unique_id: string = '';
}
