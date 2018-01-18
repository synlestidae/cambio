import {Account} from './domain/account';
import {Api} from './api';

export type CurrentPage = 'LogIn' | 'Home' | 'MyAccount';

export type LoginState = 'NotLoggedIn' | 'LoggingIn' | 'LogInFailed';

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
    }
   

    public static getGlobalState(): AppState {
        return AppState.globalState;
    }

    public log_in(username: string, password: string) {
        let that = this;
        this.api.asyncLogInUser(username, password)
            .then((e: any) => {
                that.changePage('MyAccount')
            })
            .catch((e: any) => {
                that.loginPage.loginState = 'LogInFailed'
            })
    }

    changePage(page: CurrentPage) {
        this.currentPage = page;
    }
}
