import {CurrencyCode} from './currency_code';
import {CurrencyDenom} from './currency_denom';

export class Order {
    sell_asset_type: CurrencyCode;
    sell_asset_denom: CurrencyDenom;
    sell_asset_units: number;
    buy_asset_type: CurrencyCode;
    buy_asset_denom: CurrencyDenom;
    buy_asset_units: number;

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
        let ratio: number;
        let ethDenom: string;
        let moneyDenom: string;
        let numerator;
        let denominator;
        if (order.buy_asset_type === 'NZD' && order.sell_asset_type === 'ETH') {
            moneyDenom = order.buy_asset_denom;
            ethDenom = order.sell_asset_denom;
            numerator = order.buy_asset_units;
            denominator = order.sell_asset_units;
        } else if (order.buy_asset_type === 'ETH' && order.sell_asset_type === 'NZD'){
            moneyDenom = order.sell_asset_denom;
            ethDenom = order.buy_asset_denom;
            numerator = order.sell_asset_units;
            denominator = order.buy_asset_units;
        } else {
            return null;
        }

        if (ethDenom === 'Szabo') {
            denominator /= 1000000; 
        } else if (ethDenom === 'Wei') {
            throw new Error('Cannot do arithmetic on Wei');
        }

        if (moneyDenom === 'Cent') {
            numerator /= 100;
        }

        return numerator / denominator;
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
