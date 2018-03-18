import * as React from "react";
import {ActionCreators} from './flux/action_creators';

export interface CreditCardInputProps {
    cardNumber: string;
    expiryMonth: string;
    expiryYear: string;
    cvv: string;
    actions: ActionCreators
}

export function CreditCardInput(props: CreditCardInputProps) {
    const actions = props.actions;
    const onChangeCCNumber = (e: any) => actions.changeCCDetail('CARD_NUMBER', String(e.target.value));
    const onChangeExpiryMonth = (e: any) => actions.changeCCDetail('EXPIRY_MONTH', String(e.target.value));
    const onChangeExpiryYear = (e: any) => actions.changeCCDetail('EXPIRY_YEAR', String(e.target.value));
    const onChangeCVV = (e: any) => actions.changeCCDetail('CVV', String(e.target.value));

    return (
    <div className="container">
      <div className="credit-card-details">
        <div className="cc-number">
          <label className="cc-label">CC Number</label>
          <input type="text" size={20} placeholder="0000-0000-0000-0000" className="form-control cc-input" onChange={onChangeCCNumber} value={props.cardNumber}>
          </input>
        </div>
        <div className="cc-expiry-container">
          <label className="cc-label">
            Expiry
          </label>
          <div>
            <input type="text" size={3} maxLength={2} className="form-control cc-expiry cc-input" placeholder="mm" onChange={onChangeExpiryMonth}  value={props.expiryMonth}>
            </input>
            <input type="text" size={3} maxLength={2} className="form-control cc-expiry cc-input" placeholder="yy" onChange={onChangeExpiryYear}  value={props.expiryYear}>
            </input>
          </div>
        </div>
        <div className="cvvc-container">
          <label className="cc-label">
            CVV
          </label>
          <div>
            <input type="text" size={4} maxLength={3} className="form-control cc-expiry cc-input" placeholder="cvv" onChange={onChangeCVV} value={props.cvv}>
            </input>
          </div>
        </div>
      </div>
    </div>
    );
}
