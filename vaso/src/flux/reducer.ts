import {AppState} from './app_state';
import {LoginPage} from './state/login_page';
import {Action} from './action';

export function reduce(state: AppState, action: Action): AppState {
    state = reduceLogin(state, action);
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
            case 'LOGIN_ERORR': 
                state.page.loadingState.name = 'Error';
                break;
            case 'LOGIN_SUCCESS': 
                state.page.loadingState.name = 'Success';
                break;
        }
    }
    return state;
}
