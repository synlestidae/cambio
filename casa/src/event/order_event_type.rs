#[derive(Debug, Serialize, Deserialize)]
pub enum OrderEventType {
    OrderPlaced,
    OrderAccepted,
    OrderRejected
}
