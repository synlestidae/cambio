import {LoadingStateType} from './loading_state_type';

export class LoadingState {
    public name: LoadingStateType = 'Ready';
    message: string|null = null;

    startLoading(): LoadingState {
        this.name = 'Loading';
        return this;
    }

    success(): LoadingState {
        this.name = 'Success';
        return this;
    }

    error(err?: Error): LoadingState {
        this.name = 'Error';
        this.message = null;
        if (err && err.message) {
            this.message = err.message;
        }
        return this;
    }
}
