import * as React from "react";
import {CreditCardOption} from './credit_card_option';
import {CreditCardInput} from './credit_card_input';

export function AccountLine() {
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
                <button className="btn">Credit account</button>
            </div>
            <div className="account-option">
                <button className="btn">Cash out</button>
            </div>
            <div className="account-option">
                <button className="btn">Transactions</button>
            </div>
        </div>
        <CreditCardOption></CreditCardOption>
        <CreditCardInput></CreditCardInput>
    </div>
    );
}
