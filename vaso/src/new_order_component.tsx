import * as React from "react";

interface NewOrderComponent {
}

export function NewOrderComponent() {
    return <div className="column-form">
        <div>
            <label>You want to buy</label>
            <select className="form-control">
                <option value="NZD">New Zealand Dollars</option> 
                <option value="ETH">Ethereum</option> 
            </select>
        </div>
        <div>
            <label>Buy units</label>
            <input type="number"></input> ($243.50)
        </div>
        <div>
            <label>You want to sell</label>
            <select className="form-control">
                <option value="NZD">New Zealand Dollars</option> 
                <option value="ETH">Ethereum</option> 
            </select>
        </div>
        <div>
            <label>Sell units</label>
            <input type="number"></input>  (0.50 ETH)
        </div>
        <div>
            <label>Confirm</label>
            <input type="text" value="n20dn343ja0" disabled></input>
            <input type="text"></input>
        </div>
    </div>
}
