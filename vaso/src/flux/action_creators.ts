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

export class ActionCreators {
    private readonly api: Api;
    private readonly dispatch: (action: Action) => void;

    constructor(api: Api, dispatch: (action: Action) => void) {
        this.api = api;
        this.dispatch = dispatch;
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

    public changeURL(hash: string) {
        if (hash.startsWith('#accounts')) {
            window.location.hash = '#accounts';
            this.openAccountPage();
        }
        if (hash.startsWith('#board')) {
            window.location.hash = '#board';
            this.openBoardPage();
        }
    }

    public openAccountPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'Accounts'));
        this.api.asyncGetAccounts()
            .then((accounts: Account[]) => {
                this.dispatch(new BasicAction('ADD_ACCOUNTS', null, accounts));
                for (let a of accounts) {
                    this.getAccountTransactions(a);
                }
                return accounts;
            });
    }

    public openBoardPage() {
        this.dispatch(new BasicAction('OPEN_PAGE', 'Board'));
        this.updateOrderBoard();
    }

    public setNewOrderBuyCurrency(currency: CurrencyCode) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'buy_asset_type', currency));
    }

    public setNewOrderSellCurrency(currency: CurrencyCode) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'sell_asset_type', currency));
    }
    public setNewOrderBuyUnits(units: number) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'buy_asset_units', units));
    }

    public setNewOrderSellUnits(units: number) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'sell_asset_units', units));
    }

    public setNewOrderUniqueId(uniqueId: string) {
        this.dispatch(new BasicAction('SET_NEW_ORDER', 'unique_id', uniqueId));
    }

    public setSignupFormValue(field: string, value: string) {
        this.dispatch(new BasicAction('SET_SIGNUP_FORM_VALUE', field, value));
    }

    public nextSignupForm() {
        this.dispatch(new BasicAction('NEXT_SIGNUP_FORM'));
    }

    public prevSignupForm() {
        this.dispatch(new BasicAction('PREV_SIGNUP_FORM'));
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
        } catch {
            this.dispatch(new BasicAction('ORDER_SUBMIT_FAIL'));
            // TODO
            // result is bad. check if the order appears in the board. 
            // if in the board, log the error in the console and go to step (2)
            // if not in the board, prompt the failure action
        }
    }

    public async buyOrder(order: UserOrder, uniqueId: string) {
        await this.api.asyncBuyOrder(order, uniqueId);
        this.updateOrderBoard();
    }

    public startNewOrderConfirm() {
        this.dispatch(new BasicAction('SET_NEW_ORDER_STATE', 'ReadyToConfirm'));
    }

    public newOrder() {
        this.dispatch(new BasicAction('NEW_ORDER'));
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
        //this.dispatch(new BasicAction('CHANGE_CREDIT_AMOUNT', amount));
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
