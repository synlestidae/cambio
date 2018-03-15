import {AccountPage} from './state/account_page';
import {Action} from './action';
import {AppState, PageName} from './app_state';
import {LoginPage} from './state/login_page';
import {Account} from '../domain/Account';

export function reduce(state: AppState, action: Action): AppState {
    state = reducePage(state, action);
    state = reduceLogin(state, action);
    state = reduceAccounts(state, action);
    return state;
}

function reduceLogin(state: AppState, action: Action): AppState {
    if (state.page instanceof LoginPage) {
        switch (action.name) {
            case 'LOGIN_START': 
                state.page.loadingState.name = 'Loading';
                break;
            case 'LOGIN_AUTH_FAIL': 
                state.page.loadingState.name = 'Error';
                break;
            case 'LOGIN_ERROR': 
                state.page.loadingState.name = 'Error';
                break;
            case 'LOGIN_SUCCESS': 
                state.page.loadingState.name = 'Success';
                break;
            case 'SET_EMAIL_ADDRESS':
                state.page.emailAddress = action.value;
                break;
            case 'SET_PASSWORD':
                state.page.password = action.value;
                break;
        }
    }
    return state;
}

function reduceAccounts(state: AppState, action: Action): AppState{
    if (state.page instanceof AccountPage) {
        let payload = action.payload;
        switch (action.name) {
            case 'ADD_ACCOUNTS':
                if (action.payload instanceof Array) {
                    state.page.accounts = <Account[]> action.payload;
                } else {
                    let objName = (payload.constructor && payload.constructor.name) || typeof action.payload;
                    throw new Error(`ADD_ACCOUNTS should have Account[] payload, but got ${objName}`);
                }
                break;
        }
    }
    return state;
}

function reducePage(state: AppState, action: Action): AppState {
    if (action.name === 'OPEN_PAGE') {
        if (action.value) {
            state.navigateTo(<PageName>action.value);
        } else {
            throw new Error('Cannot open page. Page name value was missing');
        }
    }
    return state;
}
