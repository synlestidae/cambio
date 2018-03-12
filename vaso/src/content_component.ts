import Vue from 'vue';
import Component from 'vue-class-component';
import {SignupPage} from './signup_page';
import {AccountPage} from './account_page';
import {AppState} from './app_state';
import {Api} from './api';

export type Page = 'LogIn' | 'MyAccount'

const template: string = `
    <div id="main-content">
      <div class="page-container signup-container" v-if="appState.currentPage === 'LogIn'">
        <signup-page></signup-page>
      </div>
      <div class="page-container account-page-container" v-if="appState.currentPage === 'MyAccount' && appState.accountPage.accounts.length > 0">
        <account-page></account-page>
      </div>
    </div>`;

Component.registerHooks(['created']);

@Component({
    template: template,
    data: {
        currentPage: String
    },
    components: {
        'signup-page': SignupPage,
        'account-page': AccountPage 
    },
    props: []
})
export class ContentComponent extends Vue {
    public appState: AppState;

    constructor() {
        super();
    }

    public beforeCreate(): void {
        this.appState = AppState.getGlobalState();
    }

    data() {
        let state = this.appState;
        return {
            appState: state
        };
    }
}
