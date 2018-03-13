import {LoadingStateType} from './loading_state_type';

export class LoadingState {
    name: LoadingStateType = 'Ready';
    message: string|null = null;
}
