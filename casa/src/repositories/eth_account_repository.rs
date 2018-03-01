use repository;
use db;
use domain;
use checkmail;
use repository::*;
use std;
use postgres;
use domain::Id;
use db::{TryFromRow, TryFromRowError};

#[derive(Clone)]
pub struct EthAccountRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> EthAccountRepository<T> {
    pub fn new(db: T) -> Self {
        EthAccountRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for EthAccountRepository<T> {
    type Item = domain::EthAccount;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        let result_match = match clause {
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::UserClause::EmailAddress(ref email_address) => self.db_helper.query(SELECT_BY_EMAIL,
                &[email_address]),
            &repository::UserClause::All(_) => self.db_helper.query(SELECT_ALL, &[]),
            _ => return Err(db::CambioError::shouldnt_happen("Invalid query to get account", 
                    &format!("Clause {:?} not supported by AccountRepository", clause)))
        };
        let result: Vec<EthRow> = try!(result_match);
        Ok(result.into_iter().map(|r| r.into()).collect())
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for EthAccountRepository<T> {
    type Item = domain::EthAccount;

    fn create(&mut self, account: &Self::Item) -> repository::ItemResult<Self::Item> {
        let account_copy: Self::Item = account.clone();
        let item: EthRow = account_copy.into();
        let err = db::CambioError::shouldnt_happen("Failed to locate account after creating it", 
            "Error during Eth account creation");
        let id_row = try!(self.db_helper.query_raw(INSERT, &[&item.owner_id, &item.address, &item.password_hash_bcrypt]));
        if id_row.len() == 0 {
            return Err(err);
        }
        let row = id_row.get(0);
        let id_match: Option<domain::Id> = row.get(0);
        match id_match {
            None => Err(err),
            Some(id) => {
                let new_account = try!(self.read(&repository::UserClause::Id(id))).pop();
                if new_account.is_some() {
                    Ok(new_account.unwrap())
                } else {
                    Err(err)
                }
            }
        }
    }
}

#[derive(TryFromRow)]
struct EthRow {
    pub id: Option<domain::Id>,
    pub address: String,
    pub password_hash_bcrypt: String,
    pub owner_id: Id, 
}

impl std::convert::Into<domain::EthAccount> for EthRow {
    fn into(self) -> domain::EthAccount {
        unimplemented!()
    }
}

impl std::convert::Into<EthRow> for domain::EthAccount {
    fn into(self) -> EthRow {
        unimplemented!()
    }
}
const SELECT_BY_ID: &'static str = "SELECT * FROM ethereum_account_details WHERE id = $1 ";
const SELECT_ALL: &'static str = "SELECT * FROM ethereum_account_details";
const SELECT_BY_EMAIL: &'static str = "
    SELECT ethereum_account_details.*
    FROM ethereum_account_details 
    JOIN account_owner ON account_owner.id = ethereum_account_details.owner_id
    JOIN users ON users.id = account_owner.user_id
    WHERE users.email_address = $1";
const INSERT: &'static str = "INSERT INTO ethereum_account_details(owner_id, address, password_hash_bcrypt) VALUES ($1, $2, $3) RETURNING id";
