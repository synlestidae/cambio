import Vue from 'vue';
import Component from 'vue-class-component';
import {Api} from './api';
import {AppState} from './app_state';

const template: string = `
  <div>Loading your accounts</div>
`


@Component({
    template: template,
    data: {
    },
    name: 'account-page',
    props: []
})
export class AccountPage extends Vue {
}
