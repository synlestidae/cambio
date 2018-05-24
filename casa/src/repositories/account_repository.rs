use db;
use domain;
use repositories::UserRepository;
use repository;
use repository::*;

#[derive(Clone)]
pub struct AccountRepository<T: db::PostgresHelper> {
    user_repository: UserRepository<T>,
    db_helper: T,
}

impl<T: db::PostgresHelper> AccountRepository<T> {
    pub fn new(db: T) -> Self {
        AccountRepository {
            user_repository: UserRepository::new(db.clone()),
            db_helper: db,
        }
    }

    fn _get_asset_id(
        &mut self,
        asset_type: &domain::AssetType,
        asset_denom: &domain::Denom,
    ) -> Result<domain::Id, db::CambioError> {
        let asset_id_vec = try!(self.db_helper.query_raw(
            "SELECT get_asset_id($1, $2) AS id",
            &[asset_type, asset_denom]
        ));
        if asset_id_vec.len() == 0 {
            return Err(db::CambioError::bad_input(
                "Unknown asset and denom combination",
                "Asset ID not found",
            ));
        }
        let asset_id_match: Option<domain::Id> = asset_id_vec.get(0).get("id");
        if asset_id_match.is_none() {
            Err(db::CambioError::format_obj(
                "Unknown asset type",
                "Unknown asset type",
            ))
        } else {
            Ok(asset_id_match.unwrap())
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for AccountRepository<T> {
    type Item = domain::Account;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        match clause {
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[&id.0]),
            &repository::UserClause::EmailAddress(ref email_address) => {
                let c = repository::UserClause::EmailAddress(email_address.to_owned());
                let users = try!(self.user_repository.read(&c));
                if users.len() == 0 {
                    return Err(db::CambioError::not_found_search(
                        "User with that email not found",
                        "User with that email not found",
                    ));
                }
                self.db_helper.query(SELECT_BY_EMAIL, &[email_address])
            }
            _ => Err(db::CambioError::shouldnt_happen(
                "Invalid query to get account",
                &format!("Clause {:?} not supported by AccountRepository", clause),
            )),
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for AccountRepository<T> {
    type Item = domain::Account;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let err = db::CambioError::shouldnt_happen(
            "Failed to locate asset account after creating it",
            "Error during asset account creation",
        );
        let asset_id: domain::Id = try!(self._get_asset_id(&item.asset_type, &item.asset_denom));
        let rows = self.db_helper.query_raw(
            INSERT,
            &[
                &item.owner_user_id,
                &asset_id,
                &item.account_type,
                &item.account_business_type,
                &item.account_role,
                &item.account_status,
            ],
        );
        let result_list = try!(rows);
        if result_list.len() == 0 {
            Err(db::CambioError::shouldnt_happen(
                "Failed to create account",
                "INSERT query didn't return ID",
            ))
        } else {
            let row = result_list.get(0);
            let id_match: Option<domain::Id> = row.get("id");
            if id_match.is_none() {
                return Err(db::CambioError::shouldnt_happen(
                    "Could not find account after creating it",
                    "Row did not return ID of account",
                ));
            }
            let id = id_match.unwrap();
            let mut accounts = try!(self.read(&repository::UserClause::Id(id)));
            match accounts.pop() {
                Some(account) => Ok(account),
                None => Err(db::CambioError::shouldnt_happen(
                    "Could not find account after creating it",
                    "read() after inserting account returned empty list",
                )),
            }
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoUpdate for AccountRepository<T> {
    type Item = domain::Account;

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let (id, owner_id) = match (&item.id, &item.owner_user_id) {
            (&Some(id), &Some(owner_id)) => (id, owner_id),
            _ => {
                return Err(db::CambioError::format_obj(
                    "Cannot update without account and owner ID",
                    "Account and/or Owner ID was None",
                ))
            }
        };
        let asset_id = try!(self._get_asset_id(&item.asset_type, &item.asset_denom));
        try!(self.db_helper.execute(
            UPDATE_BY_ID,
            &[
                &id,
                &owner_id,
                &asset_id,
                &item.account_type,
                &item.account_business_type,
                &item.account_role,
                &item.account_status
            ]
        ));
        let mut accounts = try!(self.read(&repository::UserClause::Id(asset_id)));
        match accounts.pop() {
            Some(account) => Ok(account),
            None => Err(db::CambioError::shouldnt_happen(
                "Could not locate account after update",
                "read(id) after update() returned empty Vec",
            )),
        }
    }
}

const SELECT_BY_ID: &'static str = "
    SELECT *, account.id as account_id, asset_type.asset_code as account_asset_type, asset_type.denom as denom
    FROM account 
    JOIN asset_type ON account.asset_type = asset_type.id
    WHERE account.id = $1";

const SELECT_BY_EMAIL: &'static str = "
    SELECT *, account.id as account_id, asset_type.asset_code as account_asset_type, asset_type.denom as denom
    FROM account 
    JOIN account_owner ON account.owner_id = account_owner.id 
    JOIN users ON account_owner.user_id = users.id
    JOIN asset_type ON account.asset_type = asset_type.id
    WHERE
    users.email_address = $1";

const INSERT: &'static str = "INSERT INTO 
    account(owner_id, asset_type, account_type, account_business_type, account_role, account_status) 
    VALUES($1, $2, $3, $4, $5, $6)
    RETURNING id;";

const UPDATE_BY_ID: &'static str = "UPDATE account SET 
    owner_id = $2, asset_type = $3, account_type = $4, account_business_type = $5, account_role = $6, account_status = $7
    WHERE id = $1";
