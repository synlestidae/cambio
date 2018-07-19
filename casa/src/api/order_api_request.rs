use api;

#[derive(Debug)]
pub enum OrderApiRequest {
    GetActiveOrders,
    GetUserOrders,
    PostNewOrder(api::OrderRequest),
    PostBuyOrder(api::OrderBuy),
}
