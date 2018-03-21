import {LoadingStateType} from './loading_state_type';

export class LoadingState {
    name: LoadingStateType = 'Ready';
    message: string|null = null;

    startLoading() {
        this.name = 'Loading';
    }

    success() {
        this.name = 'Success';
    }

    error(err?: Error) {
        this.name = 'Error';
        this.message = null;
        if (err && err.message) {
            this.message = err.message;
        }
    }
}
