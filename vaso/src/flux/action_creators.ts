import {Action} from './action';
import {Store} from './store';
import * as Actions from './actions';

class ActionCreators {
    private store: Store;

    constructor(store: Store) {
        this.store = store;
    }

    public startLogin() {
        this.store
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
}
