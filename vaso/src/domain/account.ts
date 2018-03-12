import {Transaction} from './transaction';
import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';

export interface Account {
    id: number,
    asset_type: string,
    asset_denom: string,
    account_status: string, 
    account_type: string, 
    account_business_type: string,
    account_role: string
}
