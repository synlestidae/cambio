import * as React from "react";

function AccountLine() {
    return <div className="account-list-item" style={{maxWidth: '500px'}}>
        <div className="currency-icon" style={{margin: '15px'}}>
            <i className="fas fa-money-bill-alt" aria-hidden="true" style={{fontSize: '40px'}}></i>
        </div>
    <div className="account-description" style={{padding: '15px'}}>
        <div style={{fontSize: '12pt'}}>Cash Wallet (NZD)</div>
        <a href="javascript: void(0)">Credit account</a>
        <a href="javascript: void(0)">Cash out</a>
        <a href="javascript: void(0)">Transactions</a>
      </div> 
    <div className="account-summary" style={{padding: '15px', 'marginLeft': 'auto'}}>
        <div style={{fontSize: '12pt'}}>$10.30</div>
        <div>$5.30 available</div>
      </div>
    </div>;
}
