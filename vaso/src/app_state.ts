import {Account} from './domain/account';
import {Api} from './api';
import {getCookie, setCookie} from './util/cookie';

export type CurrentPage = 'LogIn' | 'Home' | 'MyAccount';

export type LoginState = 'NotLoggedIn' | 'LoggingIn' | 'LogInFailed' | 'RegistrationFailed';

export abstract class Page {
}

export class LogInPage extends Page {
    username: string;
    password: string;
    loginState: LoginState = 'NotLoggedIn';
}

export class MyAccountPage extends Page {
    accounts: Account[]
}

export class AppState {
    public currentPage: CurrentPage;
    public loginPage: LogInPage;

    private api: Api;
    private static globalState: AppState = new AppState();

    constructor() {
        this.currentPage = 'LogIn';
        this.loginPage = new LogInPage();
        this.api = new Api();
        let token = getCookie('session_token');
        console.log('token!', token);
        if (token) {
            this.currentPage = 'MyAccount';
        }
    }
   

    public static getGlobalState(): AppState {
        return AppState.globalState;
    }

    public log_in(username: string, password: string) {
        let that = this;
        this.api.asyncLogInUser(username, password)
            .then((e: any) => {
                that.changePage('MyAccount')
                setCookie('session_token', e.session_token);
            })
            .catch((e: any) => {
                that.loginPage.loginState = 'LogInFailed'
            })
    }

    public signUp(username: string, password: string) {
        let that = this;
        this.api.asyncRegisterUser(username, password)
            .then((e: any) => {
                that.changePage('MyAccount');
            })
            .catch((e: any) => {
                that.loginPage.loginState = 'RegistrationFailed';
            })
    }

    changePage(page: CurrentPage) {
        this.currentPage = page;
    }
}
