import Vue from 'vue';
import Component from 'vue-class-component';
import {Api} from './api';
import {AppState} from './app_state';
import {AccountLine} from './account_line';


const template: string = `
  <div>
    <div v-if="accounts">
      You have {{accounts.length}} wallet account{{ accounts.length == 1? '' : 's'}}.
    </div>
    <div v-if="accounts">
      <div v-for="account in accounts.accounts">
          <account-line></account-line>
      </div>
    </div>
  </div>
`


Component.registerHooks(['created']);

@Component({
    template: template,
    data: {
        accounts: Array
    },
    name: 'account-page',
    components: {
        'account-line': AccountLine
    },
    props: []
})
export class AccountPage extends Vue {
    public state: AppState;

    constructor() {
        super();
    }

    public created():void {
        this.state = AppState.getGlobalState();
        console.log('created', this.state);
    }

    data() {
        let accounts = []
        if (this.state) {
            accounts = this.state.accountPage.accounts || [];
        }
        console.log('getty data', this.state);
        return {
            accounts: accounts
        };
    }
}
