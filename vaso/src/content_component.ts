import Vue from 'vue';
import Component from 'vue-class-component';
import {SignupPage} from './signup_page';
import {AppState} from './app_state';
import {Api} from './api';

export type Page = 'LogIn' | 'MyAccount'

const template: string = `
    <div id="main-content">
      <div class="signup-container" v-if="currentPage === 'LogIn'">
        <signup-page></signup-page>
      </div>
      <div class="account-page-container" v-if="currentPage === 'MyAccount'">
        <account-page></account-page>
      </div>
    </div>`;

@Component({
    template: template,
    data: {
        currentPage: String
    },
    components: {
        'signup-page': SignupPage 
    },
    props: ['appState']
})
export class ContentComponent extends Vue {
    constructor() {
        super();
        appState: AppState = AppState.getGlobalState(); 
    }

    data() {
        var state = this.appState;
        return {
            currentPage: state && state.currentPage
        };
    }
}
