import {Transaction} from './transaction';
import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';

export interface Account {
    id: string,
    balance: string,
    currency_code: CurrencyCode,
    currency_denom: CurrencyDenom,
    latest_transactions: Transaction[]
}
