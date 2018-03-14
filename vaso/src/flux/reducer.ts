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
    console.log('new state', state);
    return state;
}
