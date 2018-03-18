import * as React from "react";

export function CreditOrderInput() {
    return (
    <div className="container">
      <div className="form-row">
        <div className="form-col3">
          <label className="form-label">
            Deposit amount (NZD)
          </label>
          <input type="text" placeholder="$0.00" className="form-control cc-input">
          </input>
        </div>
        <div className="form-col3">
            <label className="form-label hidden">
                Credit account
             </label>
            <button className="btn">
                Credit account
            </button>
          </div>
        </div>
      </div>
      );
}
