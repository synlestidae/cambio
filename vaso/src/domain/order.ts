import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';

export class Order {
    sell_asset_type: CurrencyCode;
    sell_asset_denom: CurrencyDenom;
    _sell_asset_units: number;
    buy_asset_type: CurrencyCode;
    buy_asset_denom: CurrencyDenom;
    _buy_asset_units: number;

    set buy_asset_units(units: number) {
        if (units < 0) {
            return;
        }
        this._buy_asset_units = units;
    }

    get buy_asset_units() {
        return this._buy_asset_units;
    }

    set sell_asset_units(units: number) {
        if (units < 0) {
            return;
        }
        this._sell_asset_units = units;
    }

    get sell_asset_units() {
        return this._sell_asset_units;
    }

    constructor(sell_asset_type: CurrencyCode,
        sell_asset_denom: CurrencyDenom,
        sell_asset_units: number,
        buy_asset_type: CurrencyCode,
        buy_asset_denom: CurrencyDenom,
        buy_asset_units: number) {
            this.sell_asset_type = sell_asset_type; 
            this.sell_asset_denom = sell_asset_denom; 
            this.sell_asset_units = sell_asset_units; 
            this.buy_asset_type = buy_asset_type; 
            this.buy_asset_denom = buy_asset_denom; 
            this.buy_asset_units = buy_asset_units; 
    }

    public isValid() {
        let sell = this.sell_asset_type;
        let buy = this.buy_asset_type;

        let currenciesCorrect = (buy === 'NZD' && sell === 'ETH') || (buy === 'ETH' && sell === 'NZD');
        let unitsCorrect = this.buy_asset_units > 0 && this.sell_asset_units > 0;
        let hasPrice = this.getEthPrice();

        return hasPrice && currenciesCorrect;
    }

    public formatBuy() {
        return formatCurrency(this.buy_asset_type,
            this.buy_asset_denom,
            this.buy_asset_units);
    }

    public formatSell() {
        return formatCurrency(this.sell_asset_type,
            this.sell_asset_denom,
            this.sell_asset_units);
    }

    public getEthPrice(): number|null {
        let order = this;
        if (order.buy_asset_type === 'NZD' && order.sell_asset_type === 'ETH') {
            return order.buy_asset_units / order.sell_asset_units;
        } else if (order.buy_asset_type === 'ETH' && order.sell_asset_type === 'NZD'){
            return order.sell_asset_units / order.buy_asset_units;
        } else {
            throw new Error('Unknown order type');
        }
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

function formatCurrency(asset_type: CurrencyCode, denom: CurrencyDenom, units: number) {
    if (asset_type === 'NZD' && denom === 'Cent') {
        units = units / 100; 
        return `$${units}`;
    } else if (asset_type === 'ETH') {
        if (denom === 'Szabo') {
            units = units / 1000000;
        } else if (denom === 'Wei') {
            return `${units} Wei`;
        } else {
            throw new Error(`Unknown denom for ETH: ${denom}`);
        }
        return `${units} Ether`;
    } else {
        throw new Error(`Unknown asset type: ${asset_type}`);
    }
}
