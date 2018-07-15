use db;
use domain;
use domain::{Id, OrderSettlementId, AssetType, User};
use repository::{Creatable, Readable};
use services;
use web3::types::U256;
use postgres::GenericConnection;

pub struct SettlementService {
    eth_address: String
}

type SettleResult = Result<domain::OrderSettlement, db::CambioError>;

impl SettlementService {
    pub fn new(eth_address: &str) -> Self {
        Self {
            eth_address: eth_address.to_owned()
        }
    }

    pub fn create_settlement<C: GenericConnection>(
        &mut self,
        db: &mut C,
        user_id: domain::UserId,
        buying_order: &domain::Order,
        selling_order: &domain::Order,
    ) -> SettleResult {
        let settlement = domain::OrderSettlement::from(user_id, buying_order, selling_order);
        settlement.create(db)
    }

    pub fn begin_eth_transfer<C: GenericConnection>(
        &self,
        db: &mut C,
        settlement_id: OrderSettlementId,
        unique_id: &str,
        starting_user_password: String,
        max_cost_wei: U256,
    ) -> Result<domain::EthereumOutboundTransaction, db::CambioError> {
        let eth_service = services::EthereumService::new(&self.eth_address);
        let settlement = try!(settlement_id.get(db));
        if settlement.settlement_status != domain::SettlementStatus::WaitingEth {
            return Err(db::CambioError::unfair_operation(
                "Can only tranfer ETH when settlement is active.",
                &format!("Settlement status was {:?}", settlement.settlement_status),
            ));
        }
        settlement.settlement_status = domain::SettlementStatus::WaitingEth;
        let source_account = try!(self.get_eth_account(db, &settlement.selling_order));
        let dest_account = try!(self.get_eth_account(db, &settlement.buying_order));
        let selling_order = settlement.selling_order;
        if selling_order.sell_asset_type != AssetType::ETH {
            return Err(db::CambioError::format_obj(
                "Buying order must be for Szabo",
                "Error with settlement: unsupported selling type.",
            ))
        }
        let wei = U256::from(selling_order.sell_asset_units * 1000000000000);
        eth_service.register_transaction(
            &source_account,
            starting_user_password,
            &dest_account,
            wei,
            U256::from(max_cost_wei),
            unique_id,
        )
    }

    fn get_eth_account<C: GenericConnection>(
        &self,
        db: &mut C,
        order: &domain::Order,
    ) -> Result<domain::EthAccount, db::CambioError> {
        let owner_id = &order.owner_id;
        let user: User = try!(owner_id.get(db)); 
        let email_address = user.email_address.to_owned();
        owner_id.get(db)
    }

    fn get_cost(&self) -> Result<u64, db::CambioError> {
        unimplemented!()
    }
}
