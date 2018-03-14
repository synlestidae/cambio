import {Page} from './state/page';
import {LoginPage} from './state/login_page';

export class AppState {
    public loggedInAs: string|null;
    public page: Page = new LoginPage();

    private constructor() {}

    public static startState(): AppState {
        return new AppState();
    }
}
