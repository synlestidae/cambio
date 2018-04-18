import {Page} from './page';
import {LoadingState} from './loading_state';
import {UserOrder} from '../../domain/user_order';
import {NewOrder} from './new_order';

export class BoardPage implements Page {
    public readonly name: string = 'BOARD';
    public active_orders: UserOrder[] = [];
    public loadingState: LoadingState = new LoadingState();
    public sortField: string|null = null;
    public sortDir: 'asc'|'desc'|null;
    public newOrder: NewOrder|null = null;
}
