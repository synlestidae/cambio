import {Page} from './page';
import {LoadingState} from './loading_state';

export class LoginPage implements Page {
    public readonly name: string = 'LOGIN';
    public loadingState: LoadingState = new LoadingState();
    public emailAddress: string = '';
    public password: string = '';
}
