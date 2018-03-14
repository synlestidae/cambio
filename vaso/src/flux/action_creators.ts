import {Action} from './action';
import {Store} from './store';
import {BasicAction} from './action';
import * as Actions from './actions';
import {Api} from '../api';

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

    private handleLoginResolve(response: any) {
        console.log('login resolve', response);
        this.loginSuccess();
    }

    private handleLoginReject(response: any) {
        console.log('login reject', response);
        if (response.status === 401 || response.status === 403) {
            this.loginAuthFail();
        }
        this.loginError();
    }
}
