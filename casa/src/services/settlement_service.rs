use db;
use domain::Id;
use domain;
use repositories;
use repository::*;
use repository;
use services;

pub struct SettlementService<T: db::PostgresHelper> {
    settlement_repo: repositories::SettlementRepository<T>,
    eth_service: services::EthereumService<T>,
    user_repo: repositories::UserRepository<T>,
    eth_repo: repositories::EthAccountRepository<T>
}

type SettleResult = Result<domain::OrderSettlement, db::CambioError>;

impl<T: db::PostgresHelper> SettlementService<T> {
    pub fn new(db_helper: T, eth_address: &str) -> Self {
        Self {
            settlement_repo: repositories::SettlementRepository::new(db_helper.clone()),
            eth_service: services::EthereumService::new(db_helper.clone(), eth_address),
            user_repo: repositories::UserRepository::new(db_helper.clone()),
            eth_repo: repositories::EthAccountRepository::new(db_helper.clone())
        }
    }

    pub fn create_settlement(&mut self, user_id: domain::Id, buying_order: &domain::Order, 
        selling_order: &domain::Order) -> SettleResult {
        let settlement = domain::OrderSettlement::from(user_id, buying_order, selling_order);
        self.settlement_repo.create(&settlement)
    }

    pub fn begin_eth_transfer(&mut self,
            settlement_id: Id, 
            unique_id: &str, 
            starting_user_password: String,
            max_cost_wei: u64) -> SettleResult {
        let mut settlement = match try!(self.settlement_repo.read(&repository::UserClause::Id(settlement_id))).pop() {
            Some(s) => s,
            None => return Err(db::CambioError::not_found_search("Settlement could not be found.", 
                "Settlement Repo returned empty array."))
        };
        if settlement.settlement_status != domain::SettlementStatus::Settling {
            return Err(db::CambioError::unfair_operation("Can only tranfer ETH when settlement is active.", 
                &format!("Settlement status was {:?}", settlement.settlement_status)));
        }
        settlement.settlement_status = domain::SettlementStatus::WaitingEth;
        let source_account = try!(self.get_eth_account(&settlement.selling_order));
        let dest_account = try!(self.get_eth_account(&settlement.buying_order));
        let selling_order = settlement.selling_order;
        let szabo = match (selling_order.sell_asset_type, selling_order.sell_asset_denom) {
            (domain::AssetType::ETH, domain::Denom::Szabo) => selling_order.sell_asset_units,
            _ => return Err(db::CambioError::format_obj("Buying order must be for Szabo", 
                "Error with settlement: unsupported selling type."))
        };
        let wei = (szabo as u64) * 1000000000000;
        self.eth_service.register_transaction(&source_account, 
            starting_user_password,
            wei,
            max_cost_wei,
            dest_account.address,
            unique_id);
        unimplemented!()
    }

    fn get_eth_account(&mut self, order: &domain::Order) 
        -> Result<domain::EthAccount, db::CambioError> {
        let owner_id = order.owner_id;
        let clause = repository::UserClause::Id(owner_id);
        let user = try!(self.user_repo.get_owner(owner_id));
        let email_address = user.email_address.to_owned();
        let eth_clause = repository::UserClause::EmailAddress(email_address);
        let mut eth_account_match = try!(self.eth_repo.read(&eth_clause));
        let not_found_error = db::CambioError::not_found_search(
            "User does not have an Ethereum account yet.", 
            "Could not find Ethereum account.");
        eth_account_match.pop().ok_or(not_found_error)
    }

    fn get_cost(&self) -> Result<u64, db::CambioError> {
        unimplemented!()
    }
}