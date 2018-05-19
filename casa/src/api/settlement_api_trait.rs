use iron;

pub trait SettlementApiTrait {
    fn post_settlement_eth_auth(&mut self, request: &mut iron::Request) -> iron::Response;
    fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response;
}
