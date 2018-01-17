import Vue from 'vue';
import Component from 'vue-class-component';
import {Api} from './api';
import {AppState} from './app_state';

const template: string = 
    `<div class="signup-form">
          <form class="form-signin">
            <div class="form-row">
              <div>Cambio allows you to buy, sell and trade Ethereum with ease</div>
            </div>
            <div class="form-row">
              <label for="inputEmail" class="sr-only">Email address</label>
              <input type="email" id="inputEmail" class="form-control" v-model="emailAddress" placeholder="Email address" required="" autofocus="">
            </div>
            <div class="form-row">
              <label for="inputPassword" class="sr-only">Password</label>
              <input type="password" id="inputPassword" class="form-control" v-model="password" placeholder="Password" required="">
            </div>
            <div class="form-row">
              <button class="btn btn-lg btn-primary btn-block" type="submit" v-on:click.prevent="doLogIn()">Sign in</button>
            </div>
            <div class="form-row">
              Don't have an account? <a href="javascript: void">Sign up</a> 
            </div>
          </form>
      </div>`;

@Component({
    template: template,
    data: {
        emailAddress: String,
        password: String
    },
    name: 'signup-page',
    props: ['appState']
})
export class SignupPage extends Vue {
    emailAddress: string;
    password: string;
    appState: AppState;

    doLogIn(): void {
        this.appState = AppState.getGlobalState();
        this.appState.log_in(this.emailAddress, this.password);
    }

    data() {
        return {
            emailAddress: this.emailAddress,
            password: this.password
        };
    }
}
