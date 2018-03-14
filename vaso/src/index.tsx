import * as React from "react";
import * as ReactDOM from "react-dom";

import {App} from './app';

import {AppState} from './flux/app_state';
import {getDispatcher} from './flux/dispatcher';
import {reduce} from './flux/reducer';
import {Store} from './flux/store';

window.onload = () => {
    let state = AppState.startState();
    let store = new Store(state);
    let dispatch = getDispatcher(store, reduce);
    ReactDOM.render(
            <App dispatch={dispatch} store={store}/>,
            document.getElementById("app")
    );
};
