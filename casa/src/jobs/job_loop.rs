use bcrypt::verify;
use db::PostgresHelper;
use db;
use domain::OrderSettlement;
use domain::OrderSettlementId;
use domain;
use jobs::JobRequest;
use repository::{Readable, Updateable};
use repository;
use repositories::SettlementRepository;
use services::EthereumService;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use threadpool::ThreadPool;
use web3::types::U256;

pub struct JobLoop<H: PostgresHelper + Clone> {
    db_helper: H,
    eth_service: EthereumService<H>,
    threads: ThreadPool,
    rcv: Receiver<JobRequest>
}

const NUM_JOBS: usize = 10;

impl<H: PostgresHelper + Clone> JobLoop<H> {
    pub fn new(db: H, web3_address: &str) -> (Self, Sender<JobRequest>) {
        let (tx, rx) = channel();
        let threadpool = ThreadPool::new(NUM_JOBS);
        let job_loop = Self { 
            threads: threadpool,
            rcv: rx,
            eth_service: EthereumService::new(db.clone(), web3_address),
            db_helper: db, 
        };
        (job_loop, tx)
    }

    pub fn run(&mut self) {
        match self.rcv.recv() {
            Ok(job_req) => self.handle_job_req(job_req),
            Err(err) => {
                // TODO Handle this! 
            },
        }
    }

    fn handle_job_req(&mut self, job_req: JobRequest) {
        info!("Handling job {:?}", job_req);
        match job_req {
            JobRequest::BeginSettlement(settlement, password) => {
                match self.begin_settlement(settlement, password) {
                    Ok(_) => info!("Successful settlement!"),
                    Err(err) => warn!("Bad settlement! {:?}", err),
                }
            }
        }
    }

    fn begin_settlement(&mut self, sid: OrderSettlementId, password: String) -> 
        Result<(), db::CambioError> {
        let mut settlement = try!(sid.get(&mut self.db_helper));
        info!("Handling settlement ID {:?}", settlement.id);
        if settlement.settlement_status != domain::SettlementStatus::Settling {
            info!("Expected WaitingEth status, got {:?}", settlement.settlement_status);
            return Err(db::CambioError::unfair_operation(
                "Can only tranfer ETH when settlement is active.",
                &format!("Settlement status was {:?}", settlement.settlement_status),
            ));
        }
        settlement.settlement_status = domain::SettlementStatus::WaitingEth;
        try!(settlement.update(&mut self.db_helper));
        let src_account: domain::EthAccount = 
            try!(settlement.selling_order.owner_id.get(&mut self.db_helper));
        let dst_account: domain::EthAccount = 
            try!(settlement.buying_order.owner_id.get(&mut self.db_helper));

        let max_wei = match settlement.selling_order.max_wei {
            Some(wei) => wei,
            None => return Err(db::CambioError::format_obj(
                "Selling order does not specify maximum transaction cost", 
                "Selling Order max_wei was None"))
        };

        // crunch the wei
        let szabo_unit = U256::from_str("0xE8D4A51000").unwrap();
        let mut value_wei = U256::from(settlement.selling_order.sell_asset_units);
        if let domain::AssetType::ETHSzabo = settlement.selling_order.sell_asset_type {
            value_wei = match value_wei.checked_mul(szabo_unit) {
                Some(w) => w,
                None => {
                    warn!("A settlement with lots of Ether was detected. {} Szabo", value_wei);
                    return Err(db::CambioError::not_permitted(
                    "We cannot process that much Ethereum.", 
                    "Overflow occurred during conversion to Wei"))
                }
            }
        }

        let unique_id = format!("settlement_{:?}_{:?}_{:?}_{:?}_{:?}", 
            sid,
            settlement.selling_order.id,
            settlement.buying_order.id,
            settlement.selling_order.sell_asset_units,
            settlement.selling_order.sell_asset_type);

        info!("Settlement has unique ID: {}", unique_id);

        // now check the password
       let result = if let Ok(true) = verify(&password, &src_account.password_hash_bcrypt) {
            info!("Password correct, creating transaction from account {:?}", src_account);
            self.eth_service.register_transaction(
                &src_account,
                password,
                &dst_account,
                value_wei,
                U256::from(max_wei),
                &unique_id)
        } else {
            error!("Received a settlement job with the wrong password!");
            return Err(db::CambioError::shouldnt_happen(
                "The password for the Eth account was wrong.",
                "Password provided by Job was incorrect."
            ));
        };

        match result {
            Ok(transaction) => {
                info!("Settlement was successful!");
                settlement.settlement_status = domain::SettlementStatus::Settled;
            },
            Err(err) => {
                error!("Failed to communicate with Ethereum: {:?}", err);
                settlement.settlement_status = domain::SettlementStatus::EthFailed;
            }
        }

        settlement.update(&mut self.db_helper).map(|_| ())
    }

    /*fn get_eth_account<H: PostgresHelper>(&mut self, order: &domain::Order, db: &mut H) 
        -> Result<domain::EthAccount, db::CambioError> {
        let owner_id = try!(order.owner_id.get(db));
        let clause = repository::UserClause::Id(owner_id.into());
        let user = try!(owner_id.get(&mut db));
        let email_address = user.email_address.to_owned();
        let eth_clause = repository::UserClause::EmailAddress(email_address);
        let mut eth_account_match = try!(self.eth_repo.read(&eth_clause));
        let not_found_error = db::CambioError::not_found_search(
            "User does not have an Ethereum account yet.",
            "Could not find Ethereum account.",
        );
        eth_account_match.pop().ok_or(not_found_error)
    }*/
}
