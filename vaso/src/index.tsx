import * as React from "react";
import * as ReactDOM from "react-dom";

import {App} from './app';
import {AppState} from './flux/app_state';
import {ActionCreators} from './flux/action_creators';
import {getDispatcher} from './flux/dispatcher';
import {reduce} from './flux/reducer';
import {Store} from './flux/store';
import {Api} from './api';

window.onload = () => {
    let state = AppState.startState();
    let store = new Store(state);
    let dispatch = getDispatcher(store, reduce);
    let actions = new ActionCreators(new Api(), dispatch);
    actions.changeURL(window.location.hash);
    ReactDOM.render(
            <App store={store} actions={actions}/>,
            document.getElementById("app")
    );
};
