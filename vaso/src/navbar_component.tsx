import * as React from "react";

const links = ['Home', 'Board', 'Buy', 'Sell', 'My account'].map(label => ({label: label}));

export function NavbarComponent() {
    let navLinks = links.map((link: string) => (<div className="nav-element clickable">
        {link}
      </div>));
    return <div id="top-nav">
      <div class="nav-logo">
        CAMBIO Ltd.
      </div>
      {{navLinks}}
    </div>;
}
