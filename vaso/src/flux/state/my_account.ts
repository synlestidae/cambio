import {LoadingState} from './loading_state';
import {Page} from './page';
import {PersonalDetails} from '../../domain/personal_details';

export class MyAccount implements Page {
    public readonly name = 'MyAccount';
    public personalDetails: PersonalDetails|null = null;
    public loadingState = new LoadingState();
    public savingState = new LoadingState();
}
