import {Page} from './state/page';
import {LoginPage} from './state/login_page';
import {AccountPage} from './state/account_page';

export type PageName = 'Login' | 'Accounts';

export class AppState {
    public loggedInAs: string|null;
    public page: Page = new LoginPage();
    public openedPages: Map<PageName, Page> = new Map();

    private constructor() {}

    public static startState(): AppState {
        return new AppState();
    }

    public navigateTo(pageName: PageName) {
        let cachedPage = this.openedPages.get(pageName);
        if (cachedPage) {
            this.openedPages.set(pageName, this.page);
            this.page = cachedPage;
            return;
        }
        switch (pageName) {
            case 'Accounts':
                this.page = new AccountPage();
                break;
            case 'Login':
                this.page = new LoginPage();
                break;
            default: 
                throw new Error(`Unknown page name: ${pageName}`);
        }
    }
}
