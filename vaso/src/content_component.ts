import Vue from 'vue';
import Component from 'vue-class-component';
import {SignupPage} from './signup_page';
import {AppState} from './app_state';
import {Api} from './api';

export type Page = 'LogIn' | 'MyAccount'

const template: string = `
    <div id="main-content">
      <div class="signup-container" v-if="appState.currentPage === 'LogIn'">
        <signup-page></signup-page>
      </div>
      <div class="account-page-container" v-if="appState.currentPage === 'MyAccount'">
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
        'signup-page': SignupPage 
    },
    props: []
})
export class ContentComponent extends Vue {
    public appState: AppState;

    constructor() {
        super();
        console.log('content', this);
    }

    public beforeCreate(): void {
        this.appState = AppState.getGlobalState();
        console.log('the stae!', this.appState);
    }

    data() {
        let state = this.appState;
        console.log('l\'état!', state);
        return {
            appState: state
        };
    }
}
