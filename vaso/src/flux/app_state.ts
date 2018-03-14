import {Page} from './state/page';

export class AppState {
    public loggedInAs: string|null;
    public page: Page;

    private constructor() {}

    public static startState(): AppState {
        return new AppState();
    }
}
