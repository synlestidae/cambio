import * as React from "react";
import * as ReactDOM from "react-dom";

import {App} from './app';

window.onload = () => {
    ReactDOM.render(
            <App compiler="TypeScript" framework="React" />,
            document.getElementById("app")
    );
};
