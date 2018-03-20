import * as React from "react";
import {ActionCreators} from './flux/action_creators';

export interface CreditOrderInputProps {
    amount: string,
    actions: ActionCreators
}

export function CreditOrderInput(props: CreditOrderInputProps) {
    return (
    <div className="container">
      <div className="form-row">
        <div className="form-col3">
          <label className="form-label">
            Deposit amount (NZD)
          </label>
          <input type="text" placeholder="$0.00" className="form-control cc-input" 
            value={props.amount} onChange={(e: any) => props.actions.changeCreditAmount(e.target.value as string)}>
          </input>
        </div>
        <div className="form-col3">
            <label className="form-label hidden">
                Credit account
             </label>
            <button className="btn" onClick={() => props.actions.sendPayment(props.amount)}>
                Credit account
            </button>
          </div>
        </div>
      </div>
      );
}
