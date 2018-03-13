import * as React from "react";
import {NavbarComponent} from './navbar_component';
import {ContentComponent} from './content_component';
import {reduce} from './flux/reducer';

export function App() {
    return <div>
        <NavbarComponent></NavbarComponent>
        <ContentComponent></ContentComponent>
    </div>;
}
