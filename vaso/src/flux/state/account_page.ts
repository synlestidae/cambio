import {Page} from './page';
import {LoadingState} from './loading_state';
import {Account} from '../../domain/account';

export class AccountPage implements Page {
    public readonly name: string = 'ACCOUNTS';
    public loadingState: LoadingState = new LoadingState();
    public accounts: Account[]|null = null;
}
