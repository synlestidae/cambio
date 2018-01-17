import Vue from 'vue';
import Component from 'vue-class-component';
import {Api} from './api';

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
    name: 'signup-page'
})
export class SignupPage extends Vue {
    emailAddress: string;
    password: string;

    doLogIn(): void {
        let api = new Api();
        api.asyncLogInUser(this.emailAddress, this.password)
            .then((session) => console.log('session', session));
    }

    data() {
        return {
            emailAddress: this.emailAddress,
            password: this.password
        };
    }
}
