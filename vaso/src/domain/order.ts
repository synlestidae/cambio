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
}

function formatCurrency(asset_type: CurrencyCode,
    denom: CurrencyDenom,
    units: number) {
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

function formatExchangeRate() {
}
