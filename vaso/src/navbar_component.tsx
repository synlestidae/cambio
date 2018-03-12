import * as React from "react";

const links = ['Home', 'Board', 'Buy', 'Sell', 'My account'];

export function NavbarComponent() {
    let navLinks = links.map((link: string) => (<div className="nav-element clickable">
        {link}
      </div>));
    return <div id="top-nav">
      <div className="nav-logo">
        CAMBIO Ltd.
      </div>
      {{navLinks}}
    </div>;
}
