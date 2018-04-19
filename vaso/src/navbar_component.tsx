import * as React from "react";

interface Link {
    title: string,
    url: string
}

const links = [
    {title: 'Home', url: ''},
    {title: 'Board', url: '#board'},
    {title: 'My account', url: '#my_account'}
];

function makeLink(link: Link, i: number) {
    return <div key={i} className="nav-element clickable">
        <a href={link.url}>{link.title}</a>
    </div>;
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
