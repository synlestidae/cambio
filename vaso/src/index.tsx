import * as React from "react";
import * as ReactDOM from "react-dom";

import {App} from './app';

window.onload = () => {
    console.log('Rendering this gay shit');
    ReactDOM.render(
            <App />,
            document.getElementById("app")
    );
};
