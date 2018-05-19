use iron;

pub trait SettlementApiTrait {
    fn post_complete_order(&mut self, request: &iron::Request) -> iron::Response;
    fn get_settlement_status(&mut self, request: &iron::Request) -> iron::Response;
}
