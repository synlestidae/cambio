import {AccountPage, CreditAccountOption, TransactionListOption} from './state/account_page';
import {Action} from './action';
import {AppState, PageName} from './app_state';
import {LoginPage} from './state/login_page';
import {BoardPage} from './state/board_page';
import {Account} from '../domain/Account';
import {UserOrder} from '../domain/user_order';
import {Transaction} from '../domain/transaction';

export function reduce(state: AppState, action: Action): AppState {
    state = reducePage(state, action);
    state = reduceLogin(state, action);
    state = reduceAccounts(state, action);
    state = reduceOrderBoard(state, action);
    return state;
}

function reduceLogin(state: AppState, action: Action): AppState {
    if (state.page instanceof LoginPage) {
        switch (action.name) {
            case 'SIGNUP_MODE': 
                state.page.isSignup = action.value === 'SIGNUP';
                state.page.loadingState.name = 'Ready';
                break;
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
                state.page.loadingState.name = 'Ready';
                break;
            case 'SET_PASSWORD':
                state.page.password = action.value;
                state.page.loadingState.name = 'Ready';
                break;
        }
    }
    return state;
}

function reduceAccounts(state: AppState, action: Action): AppState {
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
            case 'TOGGLE_CREDIT_ACCOUNT':
                let id = String(action.value);
                if (state.page.openOptions) {
                    state.page.openOptions = null;
                    state.page.openAccount = null;
                } else {
                    state.page.openOptions = new CreditAccountOption();
                    state.page.openAccount = id;
                }
                break;
            case 'TOGGLE_TRANSACTIONS':
                state.page.openOptions = new TransactionListOption();
                break;
            case 'START_LOADING_TRANSACTIONS':
                if (state.page.openOptions instanceof TransactionListOption) {
                    state.page.openOptions.loadingState.startLoading();
                }
                break;
            case 'SUCCESS_LOADING_TRANSACTIONS':
                if (state.page.openOptions instanceof TransactionListOption) {
                    state.page.openOptions.loadingState.success();
                }
                break;

            case 'ERROR_LOADING_TRANSACTIONS':
                if (state.page.openOptions instanceof TransactionListOption) {
                    if (payload instanceof Error) {
                        state.page.openOptions.loadingState.error(payload);
                    } else {
                        state.page.openOptions.loadingState.error();
                    }
                }
                break;
            case 'SET_ACCOUNT_TRANSACTIONS':
                if (state.page.openOptions instanceof TransactionListOption) {
                    state.page.openOptions.transactions = <Transaction[]>payload;
                }
                break;
            case 'CHANGE_CC_DETAIL':
                const correctPayloads = typeof action.payload === 'string' && typeof action.value === 'string';
                if (state.page.openOptions instanceof CreditAccountOption && correctPayloads) {
                    let val = action.payload as string;
                    let details = state.page.openOptions.creditCardDetails;

                    switch (action.value) {
                        case 'CVV':
                            details.cvv = val;
                           break; 
                        case 'CARD_NUMBER':
                            details.cardNumber= val;
                            break;
                        case 'EXPIRY_MONTH':
                            details.expiryMonth = val;
                            break;
                        case 'EXPIRY_YEAR':
                            details.expiryYear = val;
                            break;
                        default: 
                            throw new Error(`Unknown CC detail field: ${action.value}`)
                    }
                }
                break;
            case 'CHANGE_CREDIT_AMOUNT': 
                const DOLLAR_PATTERN = /^(\d*(\.\d*)?)/;
                let match =  DOLLAR_PATTERN.exec(action.value);
                let amount = match[1] || '0.00';
                if (state.page.openOptions instanceof CreditAccountOption) {
                    state.page.openOptions.creditDollars = amount;
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

function reduceOrderBoard(state: AppState, action: Action): AppState  {
    if (state.page instanceof BoardPage) {
        let page = <BoardPage> state.page;
        switch (action.name) {
            case 'START_LOADING_ACTIVE_ORDERS':
                page.loadingState.startLoading();
                break;
            case 'SET_ACTIVE_ORDERS':
                console.log('paypay', action.payload);
                page.active_orders = <UserOrder[]>action.payload;
                page.loadingState.success();
                break;
            case 'ERROR_LOADING_ACTIVE_ORDERS':
                page.loadingState.error(action.payload);
                break;
            case 'SORT_ACTIVE_ORDERS':
                page.sortField = action.value;
                break;
        }
    }
    return state;
}
