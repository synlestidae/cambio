import * as React from "react";
import {Account} from './domain/account';
import {CreditCardChoice} from './credit_card_choice';
import {CreditCardInput} from './credit_card_input';
import {CreditOrderInput} from './credit_order_input';
import {ActionCreators} from './flux/action_creators';
import {AccountPage, AccountOption, CreditAccountOption, CreditCardDetails} from './flux/state/account_page';

export interface AccountLineProps {
    actions: ActionCreators,
    openOptions: AccountOption,
    account: Account,
    isOpen: boolean
}

export function AccountLine(props: AccountLineProps) {
    let options: any[] = [];
    let isCrediting = false;
    if (props.openOptions instanceof CreditAccountOption) {
        isCrediting = props.isOpen;
    }
    if (isCrediting) {
        let ccOptions = props.openOptions as CreditAccountOption;
        let ccDetails = ccOptions.creditCardDetails;
        options = [
            <CreditCardChoice></CreditCardChoice>,
            <CreditCardInput 
              actions={props.actions}
              cardNumber={ccDetails.cardNumber} 
              expiryMonth={ccDetails.expiryMonth} 
              expiryYear={ccDetails.expiryYear} 
              cvv={ccDetails.cvv}>
            </CreditCardInput>,
            <CreditOrderInput></CreditOrderInput>
        ];
    }
    let creditAccountClass = `btn ${isCrediting? 'active' : ''}`;
    let toggleCredit = (e: any) => props.actions.toggleCreditAccount(props.account);
    return (
    <div className="account-container " style={{maxWidth: '500px'}}>
        <div className="account-list-item">
            <div className="currency-icon" style={{margin: '15px'}}>
                <i className="fas fa-money-bill-alt" aria-hidden="true" style={{fontSize: '40px'}}></i>
            </div>
            <div className="account-description" style={{padding: '15px'}}>
                <div style={{fontSize: '12pt'}}>Cash Wallet (NZD)</div>
              </div> 
            <div className="account-summary" style={{padding: '15px', 'marginLeft': 'auto'}}>
                <div style={{fontSize: '12pt'}}>$10.30</div>
            </div>
        </div>
        <div className="account-options">
            <div className="account-option">
                <button className={creditAccountClass} onClick={toggleCredit}>
                  Credit account
                </button>
            </div>
            <div className="account-option">
                <button className="btn">Cash out</button>
            </div>
            <div className="account-option">
                <button className="btn">Transactions</button>
            </div>
        </div>
        {options}
    </div>
    );
}
