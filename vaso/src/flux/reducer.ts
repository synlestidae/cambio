import {AccountPage, CreditAccountOption, TransactionListOption} from './state/account_page';
import {Action} from './action';
import {AppState, PageName} from './app_state';
import {LoginPage} from './state/login_page';
import {BoardPage} from './state/board_page';
import {MyAccount} from './state/my_account';
import {Account} from '../domain/Account';
import {UserOrder} from '../domain/user_order';
import {OrderRequest} from '../domain/order_request';
import {PersonalDetails} from '../domain/personal_details';
import {Transaction} from '../domain/transaction';
import {NewOrder, OrderState} from './state/new_order';
import {RegistrationInfo} from '../domain/registration_info';

export function reduce(state: AppState, action: Action): AppState {
    state = reducePage(state, action);
    state = reduceLogin(state, action);
    state = reduceAccounts(state, action);
    state = reduceOrderBoard(state, action);
    state = reducePersonalDetails(state, action);
    return state;
}

function reduceLogin(state: AppState, action: Action): AppState {
    if (state.page instanceof LoginPage) {
        let page = state.page as LoginPage;
        let signupState = page.signupState; 
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
            case 'NEXT_SIGNUP_FORM':
                (page.signupState as any).form_state = nextPage(page.signupState.form_state);
                break;
            case 'PREV_SIGNUP_FORM':
                (page.signupState as any).form_state = prevPage(page.signupState.form_state);
                break;
            case 'SET_SIGNUP_FORM_VALUE':
                let formState = signupState.form_state;
                if (formState === 'LoginInfo') {
                    (signupState.loginInfo as any)[action.value] = action.payload; 
                } else if (formState === 'Identification') {
                    (signupState.identification as any)[action.value] = action.payload; 
                } else if (formState === 'PersonalInfo') {
                    (signupState.info as any)[action.value] = action.payload;
                } else {
                    throw new Error(`Unknown page state: ${page.signupState}`);
                }
                break;
            case 'CLEAR_DIRTY_SIGNUP_VALUE':
                signupState.dirtyFields.delete(action.value);
                break;
            case 'ADD_DIRTY_SIGNUP_VALUE':
                signupState.dirtyFields.add(action.value);
                break;
            case 'SET_REGISTRATION_INFO':
                signupState.registrationInfo = <RegistrationInfo>action.payload;
                break;
            case 'SET_CONFIRMATION_CODE':
                let matches = /(\d{0,5})/.exec(action.value);
                if (matches) {
                    signupState.confirmationCode = matches[1];
                }
                break;
        }
    }
    return state;
}

const PAGES = ['LoginInfo', 'PersonalInfo', 'ConfirmEmail', 'Identification', 'Done'];

function nextPage(name: string) {
    return PAGES[PAGES.indexOf(name) + 1]; 
}

function prevPage(name: string) {
    return PAGES[PAGES.indexOf(name) - 1]; 
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
                page.active_orders = <UserOrder[]>action.payload;
                page.loadingState.success();
                break;
            case 'ERROR_LOADING_ACTIVE_ORDERS':
                page.loadingState.error(action.payload);
                break;
            case 'SORT_ACTIVE_ORDERS':
                if (page.sortField === action.value) {
                    if (page.sortDir === null) {
                        page.sortDir = 'asc';
                    } else if (page.sortDir === 'asc') {
                        page.sortDir = 'desc';
                    } else {
                        page.sortDir = null;
                        page.sortField = null;
                        return state;
                    }
                } else {
                    page.sortDir = 'asc';
                }
                page.sortField = action.value;
                break;
            case 'NEW_ORDER':
                page.newOrder = new NewOrder(Boolean(action.payload));
                break;
            case 'EDIT_NEW_ORDER':
                if (page.newOrder.orderState === 'ReadyToConfirm') {
                    page.newOrder.orderState = 'Initial';
                } else {
                    let errMsg = `Can only handle ${action.name} if order state is ReadyToConfirm. Current state is ${page.newOrder.orderState}`;
                    throw new Error(errMsg);
                }
                break;
            case 'CANCEL_NEW_ORDER':
                page.newOrder = null;
                break;
            case 'SET_ORDER_REQUEST': 
                if (action.payload instanceof OrderRequest) {
                    page.newOrder.order = action.payload;
                } else {
                    throw new Error('SET_ORDER_REQUEST has incorrect payload type');
                }
                break;
            case 'SET_NEW_ORDER': 
                (page.newOrder.order as any)[action.value] = action.payload;
                break;
            case 'SET_NEW_ORDER_STATE':
                let order = page.newOrder.order;
                if (page.newOrder.orderState === 'Initial' && action.value === 'ReadyToConfirm') {
                    if (!order.isValid()) {
                        page.newOrder.showValidation = true;
                        return state;
                    }
                }
                page.newOrder.orderState = <OrderState>action.value;
                break;
            case 'BEGIN_SUBMITTING_ORDER':
                page.newOrder.orderState = 'Submitting';
                break;
            case 'ORDER_SUBMIT_SUCCESS':
                page.newOrder.orderState = 'Success';
                break;
            case 'ORDER_SUBMIT_CLEAR':
                page.newOrder = null;
                break;
            case 'ORDER_SUBMIT_FAIL':
                page.newOrder.orderState = 'Failed';
                break;
        }
    }
    return state;
}

function reducePersonalDetails(state: AppState, action: Action): AppState  {
    console.log('reducing personal details', action);
    console.log(state);
    if (state.page instanceof MyAccount) {
        let page = <MyAccount>state.page;
        switch (action.name) {
            case 'START_LOADING_PERSONAL_DETAILS':
                page.loadingState.startLoading();
                break;
            case 'SUCCESS_LOADING_PERSONAL_DETAILS':
                page.loadingState.success();
                break;
            case 'SET_PERSONAL_DETAILS':
                if (action.payload instanceof PersonalDetails) {
                    page.personalDetails = action.payload; 
                } else {
                    throw new Error('Saving details was successful, but did not get personal details obj');
                }
                break;
            case 'ERROR_LOADING_PERSONAL_DETAILS':
                if (action.payload instanceof Error) {
                    page.loadingState.error(action.payload);
                } else {
                    page.loadingState.name = 'Error';
                }
                break;
            case 'START_SUBMITTING_PERSONAL_DETAILS':
                page.savingState.startLoading();
                break;
            case 'SUCCESS_SUBMITTING_PERSONAL_DETAILS':
                if (action.payload instanceof PersonalDetails) {
                    page.personalDetails = action.payload; 
                    page.savingState.success();
                } else {
                    throw new Error('Saving details was successful, but did not get personal details obj');
                }
                break;
            case 'ERROR_SUBMITTING_PERSONAL_DETAILS':
                if (action.payload instanceof Error) {
                    page.savingState.error(action.payload);
                } else {
                    page.savingState.name = 'Error';
                }
                break;
        }
    }
    return state;
}
