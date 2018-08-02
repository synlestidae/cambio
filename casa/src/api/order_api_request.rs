use api;

#[derive(Debug)]
pub enum OrderApiRequest {
    GetActiveOrders,
    GetChangedOrders(api::LastChange),
    GetUserOrders,
    PostNewOrder(api::OrderRequest),
    PostBuyOrder(api::TradeRequest),
}
