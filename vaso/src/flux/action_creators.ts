import {Action} from './action';
import {Store} from './store';
import {BasicAction} from './action';
import * as Actions from './actions';
import {Api} from '../api';
import {Account} from '../domain/account';
import {DollarPayment} from '../domain/payment';
import {CurrencyCode} from '../domain/currency_code';
import {OrderRequest} from '../domain/order_request';
import {UserOrder} from '../domain/user_order';
import {PersonalDetails} from '../domain/personal_details';
import {SignupState, SignupInfo, PersonalInfo, IdentificationInfo} from './state/signup_state';

export class ActionCreators {
    private readonly api: Api;
    private readonly dispatch: (action: Action) => void;

    constructor(api: Api, dispatch: (action: Action) => void) {
        this.api = api;
        this.dispatch = dispatch;
    }

    public async initialise(hash: string) {
        let sessionToken = localStorage.getItem('session_token');
        console.log('Session is', sessionToken);
        if (!sessionToken) {
            console.log('No session');
            return;
        }
        console.log('Trying to get accounts');
        try {
            await this.api.asyncGetAccounts();
        } catch (e) {
            console.log('Error getting accounts!', e);
            this.api.sessionToken = null;
            this.changeURL('');
            return;
        }
        this.changeURL(hash);
    }

    public loginError() {
        this.dispatch(Actions.LOGIN_ERROR);
    }

    public loginAuthFail() {
        this.dispatch(Actions.LOGIN_AUTH_FAIL);
    }

    public loginSuccess() {
        this.dispatch(Actions.LOGIN_SUCCESS);
    }

    public signupMode() {
        this.dispatch(new BasicAction('SIGNUP_MODE', 'SIGNUP'));
    }

    public loginMode() {
        this.dispatch(new BasicAction('SIGNUP_MODE', 'LOGIN'));
    }

    public setEmailAddress(emailAddress: string) {
        this.dispatch(new BasicAction('SET_EMAIL_ADDRESS', emailAddress));
    }

    public setPassword(password: string) {
        this.dispatch(new BasicAction('SET_PASSWORD', password));
    }

    public submitLogin(email: string, password: string) {
        return this.api.asyncLogInUser(email, password)
            .then((r: any) => this.handleLoginResolve(r))
            .catch((r: any) => this.handleLoginReject(r));
    }

    public submitSignup(email: string, password: string) {
        return this.api.asyncRegisterUser(email, password)
            .then((r: any) => this.handleLoginResolve(r))
            .catch((r: any) => this.handleLoginReject(r));
    }

    public resendEmail(email: string, identifierCode: string) {
        return this.api.asyncResendEmail(email, identifierCode);
    }

    public setConfirmationCode(code: string) {
        this.dispatch(new BasicAction('SET_CONFIRMATION_CODE', code));
    }

    public changeURL(hash: string) {
        if (hash === '') {
            this.openLandingPage();
        }
        if (hash.startsWith('#accounts')) {
            window.location.hash = '#accounts';
            this.openAccountPage();
        }
        if (hash.startsWith('#board')) {
            window.location.hash = '#board';
            this.openBoardPage();
        }
        if (hash.startsWith('#myaccount')) {
            window.location.hash = '#myaccount';
            this.openMyAccountPage();
        }
    }

    public async openAccountPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'Accounts'));
        let accounts = await this.api.asyncGetAccounts()
        this.dispatch(new BasicAction('ADD_ACCOUNTS', null, accounts));
        for (let a of accounts) {
            this.getAccountTransactions(a);
        }
        return accounts;
    }

    public openLandingPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'Login'));
    }

    public openBoardPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'Board'));
        this.updateOrderBoard();
    }

    public openMyAccountPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'MyAccount'));
        this.loadPersonalDetails();
    }

    public setOrderRequest(orderRequest: OrderRequest) {
        this.dispatch(new BasicAction('SET_ORDER_REQUEST', null, orderRequest));
    }

    public setNewOrderUniqueId(uniqueId: string) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'unique_id', uniqueId));
    }

    public setSignupFormValue(field: string, value: string) {
        this.dispatch(new BasicAction('SET_SIGNUP_FORM_VALUE', field, value));
    }

    public clearDirtyValue(field: string) {
        this.dispatch(new BasicAction('CLEAR_DIRTY_SIGNUP_VALUE', field));
    }

    public addDirtyValue(field: string) {
        this.dispatch(new BasicAction('ADD_DIRTY_SIGNUP_VALUE', field));
    }

    public nextSignupForm() {
        this.dispatch(new BasicAction('NEXT_SIGNUP_FORM'));
    }

    public prevSignupForm() {
        this.dispatch(new BasicAction('PREV_SIGNUP_FORM'));
    }

    public async sendRegistration(login: SignupInfo, info: PersonalInfo) {
        let registration = await this.api.asyncRegisterUser(login.email_address, login.password);
        this.dispatch(new BasicAction('SET_REGISTRATION_INFO', null, registration));
    }

    public async confirmRegistration(signupState: SignupState) {
        await this.api.asyncConfirmRegistration(
            signupState.confirmationCode,
            signupState.registrationInfo.identifierCode,
            signupState.loginInfo, 
            signupState.info, 
            signupState.identification
        );
    }

    public editNewOrder() {
        this.dispatch(new BasicAction('EDIT_NEW_ORDER'));
    }

    public cancelNewOrder() {
        this.dispatch(new BasicAction('CANCEL_NEW_ORDER'));
    }

    public async confirmNewOrder(order: OrderRequest) {
        if (!order.isValid()) {
            throw new Error('Order is invalid and cannot be sent');
        }
        this.dispatch(new BasicAction('BEGIN_SUBMITTING_ORDER'));
        try {
            let orderResult = await this.api.asyncPostOrder(order);
            this.updateOrderBoard();
            this.dispatch(new BasicAction('ORDER_SUBMIT_SUCCESS'));
        } catch (e) {
            console.error('Error posting order', e);
            this.dispatch(new BasicAction('ORDER_SUBMIT_FAIL'));
            // TODO
            // result is bad. check if the order appears in the board. 
            // if in the board, log the error in the console and go to step (2)
            // if not in the board, prompt the failure action
        }
    }

    public clearOrder() {
        this.dispatch(new BasicAction('ORDER_SUBMIT_CLEAR'));
    }

    public async buyOrder(order: UserOrder, uniqueId: string) {
        await this.api.asyncBuyOrder(order, uniqueId);
        this.updateOrderBoard();
    }

    public startNewOrderConfirm() {
        this.dispatch(new BasicAction('SET_NEW_ORDER_STATE', 'ReadyToConfirm'));
    }

    public newOrder(isBuy: boolean) {
        this.dispatch(new BasicAction('NEW_ORDER', null, isBuy));
    }

    public toggleCreditAccount(account: Account) {
        this.dispatch(new BasicAction('TOGGLE_CREDIT_ACCOUNT', account.id.toString()));
    }

    public toggleTransactions(account: Account) {
        this.dispatch(new BasicAction('TOGGLE_TRANSACTIONS', account.id.toString()));
        this.getAccountTransactions(account);
    }

    public changeCCDetail(field: string, value: string) {
        this.dispatch(new BasicAction('CHANGE_CC_DETAIL', field, value));
    }

    public changeCreditAmount(amount: string) {
        this.dispatch(new BasicAction('CHANGE_CREDIT_AMOUNT', amount));
    }

    public sendPayment(amount: string) {
        this.api.asyncPostPayment(new DollarPayment(parseFloat(amount) * 100));
    }

    public sortOrders(field: string) {
        this.dispatch(new BasicAction('SORT_ACTIVE_ORDERS', field));
    }

    public async getAccountTransactions(account: Account) {
        this.dispatch(new BasicAction('START_LOADING_TRANSACTIONS'));
        try {
            let transactions = await this.api.asyncGetAccountTransactions(account.id.toString());
            this.dispatch(new BasicAction('SET_ACCOUNT_TRANSACTIONS', account.id.toString(), transactions));
            this.dispatch(new BasicAction('SUCCESS_LOADING_TRANSACTIONS'));
        } catch (e) {
            this.dispatch(new BasicAction('ERROR_LOADING_TRANSACTIONS', null, e));
        }
    }

    public async updateOrderBoard() {
        let ordersPromise = this.api.asyncGetActiveOrders(); 
        this.dispatch(new BasicAction('START_LOADING_ACTIVE_ORDERS'));
        try {
            let orders = await ordersPromise; 
            this.dispatch(new BasicAction('SET_ACTIVE_ORDERS', null, orders));
        } catch (e) {
            this.dispatch(new BasicAction('ERROR_LOADING_ACTIVE_ORDERS', null, e));
        }
    
    }

    public async loadPersonalDetails() {
        this.dispatch(new BasicAction('START_LOADING_PERSONAL_DETAILS'));
        try {
            let personalDetails = await this.api.asyncGetPersonalDetails();
            this.dispatch(new BasicAction('SUCCESS_LOADING_PERSONAL_DETAILS'));
            this.dispatch(new BasicAction('SET_PERSONAL_DETAILS', null, personalDetails));
        } catch (e) {
            console.error('There was an error!');
            console.error(e);
            this.dispatch(new BasicAction('ERROR_LOADING_PERSONAL_DETAILS', null, e));
        }
    }

    public setPersonalDetails(personalDetails: PersonalDetails) {
        this.dispatch(new BasicAction('SET_PERSONAL_DETAILS', null, personalDetails));
    }

    public async updatePersonalDetails(personalDetails: PersonalDetails) {
        this.dispatch(new BasicAction('START_SUBMITTING_PERSONAL_DETAILS'));
        try {
            await this.api.asyncPostPersonalDetails(personalDetails);
            this.dispatch(new BasicAction('SUCCESS_SUBMITTING_PERSONAL_DETAILS'));
            this.dispatch(new BasicAction('SET_PERSONAL_DETAILS', null, personalDetails));
        } catch (e) {
            this.dispatch(new BasicAction('ERROR_SUBMITTING_PERSONAL_DETAILS', null, e));
        }
    }

    public creditAccount(accountId: string) {
    }

    public cashOutAccount(accountId: string) {
    }

    private handleLoginResolve(response: any) {
        this.loginSuccess();
        this.changeURL('#accounts');
    }

    private handleLoginReject(response: any) {
        if (response.status === 401 || response.status === 403) {
            this.loginAuthFail();
        }
        this.loginError();
    }
}
