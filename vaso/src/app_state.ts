import {Account} from './domain/account';
import {Api} from './api';

export type CurrentPage = 'LogIn' | 'Home' | 'MyAccount';

export abstract class Page {
}

export class LogInPage extends Page {
    username: string;
    password: string;
}

export class MyAccountPage extends Page {
    accounts: Account[]
}

export class AppState {
    currentPage: CurrentPage = 'LogIn';
    api: Api;
    loginPage: Page;
    private static globalState: AppState = new AppState();

    constructor() {
        console.log('loggy boy', LogInPage, MyAccountPage);
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
            .then(() => that.changePage('MyAccount'))
    }

    changePage(page: CurrentPage) {
        this.currentPage = page;
    }
}
