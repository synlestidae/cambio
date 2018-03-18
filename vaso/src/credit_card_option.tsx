import * as React from "react";

export function CreditCardOption() {
    return <div className="credit-card-options container">
        <div className="cc-option">
            <i className="fab fa-cc-visa cc-icon"></i>
            <input className="form-control cc-radio" type="radio"></input>
        </div>
        <div className="cc-option">
            <i className="fab fa-cc-visa cc-icon"></i>
            <input className="form-control cc-radio" type="radio"></input>
        </div>
    </div>;
}
