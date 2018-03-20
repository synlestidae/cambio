 export interface Payment {
    id: string|null,
    unique_id: string,
    asset_type: string,
    asset_denom: string,
    datetime_payment_made: Date,
    payment_method: string,
    vendor: string,
    user_credit: number
}

export class DollarPayment implements Payment {
    id: string|null = null;
    unique_id = new Date().toString();
    asset_type = 'NZD';
    asset_denom = 'Cent';
    datetime_payment_made = new Date();
    payment_method = 'CreditCard';
    vendor = 'Poli';
    user_credit = 0;

    constructor(credit: number) {
        this.user_credit = credit;
    }
}
