import {AppState} from './app_state';

export class Store {
    private state: AppState;
    private listeners: Array<(state: AppState) => void> = [];

    constructor(state: AppState) {
        this.state = state;
    }

    public updateState(state: AppState) {
        this.state = state;
        for (let callback of this.listeners) {
            try {
                callback(this.state);
            } catch (e) {
                console.error(e);
            }
        }
    }

    public getState(): AppState {
        return this.state;
    }

    public subscribe(listener: (state: AppState) => void) {
        this.unsubscribe(listener);
        this.listeners.push(listener);
    }

    public unsubscribe(listener: (state: AppState) => void) {
        this.listeners = this.listeners.filter((l: (state: AppState) => void) => l != listener);
    }
}
