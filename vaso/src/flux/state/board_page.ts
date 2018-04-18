import {Page} from './page';
import {LoadingState} from './loading_state';
import {UserOrder} from '../../domain/user_order';

export class BoardPage implements Page {
    public readonly name: string = 'BOARD';
    public active_orders: UserOrder[] = [];
    public loadingState: LoadingState = new LoadingState();
    public sortField: string|null = null;
}
