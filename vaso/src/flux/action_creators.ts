import {Action} from './action';
import {Store} from './store';
import {BasicAction} from './action';
import * as Actions from './actions';

export class ActionCreators {
    public startLogin() {
        throw new Error();
    }

    public loginError(): Action {
        return Actions.START_LOGIN;
    }

    public loginAuthFail(): Action {
        return Actions.LOGIN_AUTH_FAIL;
    }

    public loginSuccess(): Action {
        return Actions.LOGIN_SUCCESS;
    }

    public setEmailAddress(emailAddress: string): Action {
        return new BasicAction('SET_EMAIL_ADDRESS', emailAddress);
    }

    public setPassword(password: string): Action {
        return new BasicAction('SET_PASSWORD', password);
    }
}
