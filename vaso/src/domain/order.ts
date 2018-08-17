import {CurrencyCode} from './currency_code';
import * as bigInt from 'big-integer';

export class Order {
    id: string = '';
    uniqueId: string = '';
    amountFiat: string = '';
    amountCrypto: string = '';
    fiatType: CurrencyCode = 'NZD';
    cryptoType: CurrencyCode = 'ETH';
    expiresAt: Date = new Date();
    orderStatus: string = ''; 
    isBuy: boolean = true;

    private constructor() {
    }

    public static parse(json: any): Order {
        const throwErr = (msg: string) => {
            throw new Error(msg)
        };
        let order = new Order();
        order.id = json.id.toString();
        order.uniqueId = json.unique_id.toString();
        order.amountFiat = json.amount_fiat.toString();
        order.amountCrypto = json.amount_crypto.toString();
        order.fiatType = json.fiat_type === 'NZD'? 'NZD' : throwErr('Unknown fiat type');
        order.cryptoType = json.crypto_type === 'Ether'? 'ETH' : throwErr('Unknown crypto type');
        order.expiresAt = new Date(json.expires_at || throwErr('No expiry time'));
        order.orderStatus = json.status.toString();
        order.isBuy = json.trade_type.toString() === 'BuyCrypto';
        return order;
    }

    public getSellCurrency(): CurrencyCode {
        return this.getCurrency(!this.isBuy);
    }

    public getBuyCurrency(): CurrencyCode {
        return this.getCurrency(this.isBuy);
    }

    private getCurrency(flag: boolean): CurrencyCode {
        if (flag) {
            return this.fiatType;
        } else {
            return this.cryptoType;
        }
    }

    public formatBuy() {
        return '?';
    }

    public formatSell() {
        return '?';
    }

    public getEthPrice(): number|null {
        return 0;
    }

    public formatPrice(): string|null {
        let order = this;
        let price = this.getEthPrice();
        if (price === null) {
            return null;
        }
        let priceWithDP = price.toFixed(4); 
        return `$${priceWithDP}`;
    }
}
