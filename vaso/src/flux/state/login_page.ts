import {Page} from './page';
import {LoadingState} from './loading_state';
import {SignupState} from './signup_state';

export class LoginPage implements Page {
    public readonly name: string = 'LOGIN';
    public loadingState: LoadingState = new LoadingState();
    public emailAddress: string = '';
    public password: string = '';
    public isSignup = false;
    public signupState = new SignupState();
}
