import {Order} from './Order';

export class BoardUpdate {
    public from_datetime: Date;
    public to_datetime: Date;
    public orders: Order[];

    private constructor() {
    }

    public static parse(json: any) {
        let from_datetime = new Date(json['from'].toString());
        let to_datetime = new Date(json['to'].toString());
        let orders: Order[] = json.affected_orders.map((o: any) => Order.parse(o));
        return {
            from_datetime: from_datetime,
            to_datetime: to_datetime,
            orders: orders
        };
    }
}
