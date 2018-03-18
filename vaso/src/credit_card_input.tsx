import * as React from "react";

export class CreditCardInputProps {
    private _cardNumber: string = '';
    private _expiryMonth: string = '' ;
    private _expiryYear: string = '';
    private _cvv: string = '';

    get cardNumber() {
        return this._cardNumber;
    }

    set cardNumber(cardNumber: string) {
        cardNumber = cardNumber.replace(/[^0-9]/g, '');
        cardNumber = cardNumber.replace(/\d{4}/g, (x:string) => x + '-');
        if (cardNumber[cardNumber.length - 1] === '-') {
            cardNumber = cardNumber.substr(0, cardNumber.length - 1);
        }
        this._cardNumber = cardNumber;
    }
}

export function CreditCardInput() {
    return (
    <div className="container">
      <div className="credit-card-details">
        <div className="cc-number">
          <label className="cc-label">CC Number</label>
          <input type="text" placeholder="0000-0000-0000-0000" className="form-control cc-input">
          </input>
        </div>
        <div className="cc-expiry-container">
          <label className="cc-label">
            Expiry
          </label>
          <div>
            <input type="text" size={3} maxLength={2} className="form-control cc-expiry cc-input" placeholder="mm">
            </input>
            <input type="text" size={3} maxLength={2} className="form-control cc-expiry cc-input" placeholder="yy">
            </input>
          </div>
        </div>
        <div className="cvvc-container">
          <label className="cc-label">
            CVV
          </label>
          <div>
            <input type="text" size={4} maxLength={3} className="form-control cc-expiry cc-input" placeholder="cvv">
            </input>
          </div>
        </div>
      </div>
    </div>
    );
}
