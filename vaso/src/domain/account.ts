import {Transaction} from './transaction';
import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';
import {getString} from './get_string';

export class Account {
    id: string;
    asset_type: string;
    status: string; 
    business_type: string; 
    role: string;
    balance: string;

    constructor(id: string,
        asset_type: string,
        status: string,
        business_type: string,
        role: string,
        balance: string) {
        this.id = id;
        this.asset_type = asset_type;
        this.status = status;
        this.business_type = business_type;
        this.role = role;
        this.balance = balance;
    }

    public static parse(json: any): Account {
        return new Account(
            getString(json, 'id'),
            getString(json, 'asset_type'),
            getString(json, 'account_business_type'),
            getString(json, 'account_status'),
            getString(json, 'account_role'),
            '0.00'
        ); 
    }
}
