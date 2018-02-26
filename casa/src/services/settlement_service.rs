use db;
use domain;

pub struct SettlementService<T: db::PostgresHelper> {
    settlement_repo: repositories::SettlementRepository<T>,
    eth_service: services::EthereumService<T>
}

type SettleResult = Result<domain::OrderSettlement, db::CambioError>;

impl<T: db::PostgresHelper> SettlementService<T> {
    pub fn new(db_helper: T) -> Self {
        Self {
            settlement_repo: repositories::SettlementRepository::new(db_helper.clone()),
            eth_service: services::EthereumService::new(db_helper)
        }
    }

    pub fn create_settlement(&self, user_id: &domain::Id, buying_order: &domain::Order, 
        selling_order: &domain::Order) -> SettleResult {
        let settlement = domain::OrderSettlement::from(user_id, buying_order, selling_order);
        self.settlement_repo.create(&settlement)
    }


    pub fn begin_eth_transfer(&self, settlement_id: unique_id, Id, starting_user_password: String, unique_id: &str, max_cost_wei: u64) -> SettleResult {
        let settlement = try!(self.settlement_repo.read(repository::UserClause::Id(settlement_id)));
        if settlement.status !== domain::Settling {
            return Err(db::CambioError::unfair_operation("Can only tranfer ETH when settlement is active.", 
                format!("Settlement status was {}", settlement.status)));
        }
        settlement.status = domain::SettlementStatus::WaitingEth;
        let eth_account: domain::EthAccount = unimplemented!();
        let selling_order = settlement.selling_order;
        let szabo = match (selling_order.sell_asset_type, selling_order.sell_asset_denom) {
            (domain::AssetType::Eth, domain::denom::Szabo) => selling_order.sell_asset_units,
            _ => return Err(db::CambioError::format_obj("Buying order must be for Szabo", 
                "Error with settlement: unsupported selling type."))
        }
        let wei = szabo * 1000000000000;
        self.eth_service.register_transaction(eth_account, 
            starting_user_password,
            wei,
            max_cost_wei,
            eth_account.address,

    }

    fn get_cost(&self) -> Result<u64, db::CambioError> {
    }
}
