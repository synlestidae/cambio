use domain::Order;

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderEvent {
    OrderPlaced(Order),
    OrderAccepted(Order),
    OrderRejected(Order)
}
