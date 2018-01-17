import Vue from 'vue';
import Component from 'vue-class-component';
import {SignupPage} from './signup_page';
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
    data: {},
    components: {
        'signup-page': SignupPage 
    },
    props: ['currentPage']
})
export class ContentComponent extends Vue {
    currentPage: Page = 'LogIn';

    constructor() {
        super();
        console.log('content boi', this);
    }

    data() {
        return {};
    }
}
