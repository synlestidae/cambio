use domain::Order;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEvent {
    pub order: Order, 
    pub event_type: OrderEventType
}

impl EventKey for OrderEvent {
    fn key(&self) -> MessageKey {
        MessageKey(serde_json::to_string(self.order.id).into_bytes())
    }
}

pub enum OrderEventType {
    OrderPlaced,
    OrderAccepted,
    OrderRejected
}
