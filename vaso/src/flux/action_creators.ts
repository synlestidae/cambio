import {Action} from './action';
import {Store} from './store';
import {BasicAction} from './action';
import * as Actions from './actions';
import {Api} from '../api';
import {Account} from '../domain/account';
import {DollarPayment} from '../domain/payment';

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

    public toggleCreditAccount(account: Account) {
        this.dispatch(new BasicAction('TOGGLE_CREDIT_ACCOUNT', account.id.toString()));
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

    public async getAccountTransactions(account: Account) {
        let transactions = await this.api.asyncGetAccountTransactions(account.id.toString());
        console.log('principals a tranny', transactions);
        this.dispatch(new BasicAction('SET_ACCOUNT_TRANSACTIONS', account.id.toString(), transactions));
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
