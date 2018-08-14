import * as React from "react";

interface Link {
    title: string,
    url: string
}

const links = [
    {title: 'Home', url: ''},
    {title: 'Board', url: '#board'},
    {title: 'My account', url: '#myaccount'}
];

function makeLink(link: Link, i: number) {
    return <a href={link.url} className="nav-link" key={i}>
        <div key={i} className="nav-element clickable">
          {link.title}
        </div>
    </a>;
}

export function NavbarComponent() {
    let navLinks = links.map(makeLink);
    return <div id="top-nav">
      <div className="nav-logo">
        CAMBIO Ltd.
      </div>
      {navLinks}
    </div>;
}
