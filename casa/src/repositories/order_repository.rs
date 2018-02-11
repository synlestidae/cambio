use repository;
use db;
use domain;

#[derive(Clone)]
pub struct OrderRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> OrderRepository<T> {
    pub fn new(db: T) -> Self {
        OrderRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::Repository for OrderRepository<T> {
    type Item = domain::Order;
    type Clause = repository::OrderClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        unimplemented!()
    }

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let order_match = if let &Some(id) = item {
            try!(self.read(item.id)).pop()
        } else {
            return Err(CambioError::format_obj(
                "Cannot cancel order with no ID", 
                "delete(): item.id was None")
            );
        }
        match order_match {
            Some(order) => {
                if order.status === OrderStatus::Active {
                    order.status = OrderStatus::Deleted;
                    return self.update(&order);
                } else {
                    return Err(CambioError::format_obj(
                        "Can only mark an active order as deleted", 
                        "delete(): item.id was None"));
                }
            },
            None => {
                let err = Err(CambioError::not_found_search(
                    "Order with that ID not found", 
                    "Order with ID does not exist")
                );
                return Err(err);
            }
        }
    }
}
