import {Page} from './page';
import {LoadingState} from './loading_state';
import {Account} from '../../domain/account';

export type AccountOption = CreditAccountOption | CashOutOption | TransactionListOption | null;

export class AccountPage implements Page {
    public readonly name: string = 'ACCOUNTS';
    public loadingState: LoadingState = new LoadingState();
    public accounts: Account[]|null = null;

    public openAccount: string|null;
    public openOptions: AccountOption;
}

export class CreditAccountOption {
    public creditDollars: string;
    public creditCardDetails: CreditCardDetails = new CreditCardDetails();
} 

export class CashOutOption {

} 

export class TransactionListOption {

}

export class CreditCardDetails {
    private _cardNumber: string = '';
    private _expiryMonth: string = '' ;
    private _expiryYear: string = '';
    private _cvv: string = '';

    public get cardNumber(): string {
        return this._cardNumber;
    }

    public set cardNumber(ccNumber: string) {
        this._cardNumber = formatCC(filterNumbers(ccNumber));
    }

    public get expiryMonth(): string {
        return this._expiryMonth;
    }

    public set expiryMonth(mm: string) {
        this._expiryMonth = filterNumbers(mm).substring(0, 2);
    }

    public get expiryYear(): string {
        return this._expiryYear;
    };
    
    public set expiryYear(year: string) {
        this._expiryYear = filterNumbers(year).substring(0, 2);
    };

    public get cvv(): string {
        return this._cvv;
    };

    public set cvv(cvv: string) {
        this._cvv = filterNumbers(cvv).substring(0, 3);
    };
}

function filterNumbers(numbers: string): string {
    return numbers.replace(/[^0-9]/g, '');
}

function formatCC(cardNumber: string): string {
    cardNumber = cardNumber.replace(/\d{4}/g, (x:string) => x + '-');
    if (cardNumber[cardNumber.length - 1] === '-') {
        cardNumber = cardNumber.substr(0, cardNumber.length - 1);
    }
    cardNumber = cardNumber.substring(0, 20);
    return cardNumber;
}
