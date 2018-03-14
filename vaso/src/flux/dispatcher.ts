import {Store} from './store';
import {Action} from './action';
import {AppState} from './app_state';

export function getDispatcher(store: Store, reducer: (state: AppState, action: Action) => AppState) {
    return function dispatch(action: Action|Promise<Action>) {
        if (!(action instanceof Promise)) {
            let newState = reducer(store.getState(), action);
            store.updateState(newState);
        } else {
            action.then((a: Action) => store.updateState(reducer(store.getState(), a)));
        }
    }
}
