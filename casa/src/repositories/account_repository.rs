use db;
use domain;
use repositories::UserRepository;
use repository;
use repository::*;

#[derive(Clone)]
pub struct AccountRepository<T: db::PostgresHelper + Clone> {
    user_repository: UserRepository<T>,
    db_helper: T,
}

impl<T: db::PostgresHelper + Clone> AccountRepository<T> {
    pub fn new(db: T) -> Self {
        AccountRepository {
            user_repository: UserRepository::new(db.clone()),
            db_helper: db,
        }
    }
}

impl<T: db::PostgresHelper + Clone> repository::RepoRead for AccountRepository<T> {
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

impl<T: db::PostgresHelper + Clone> repository::RepoCreate for AccountRepository<T> {
    type Item = domain::Account;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let err = db::CambioError::shouldnt_happen(
            "Failed to locate asset account after creating it",
            "Error during asset account creation",
        );
        let rows = self.db_helper.query_raw(
            INSERT,
            &[
                &item.owner_user_id,
                &item.asset_type,
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

impl<T: db::PostgresHelper + Clone> repository::RepoUpdate for AccountRepository<T> {
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
        try!(self.db_helper.execute(
            UPDATE_BY_ID,
            &[
                &id,
                &owner_id,
                &item.asset_type,
                &item.account_type,
                &item.account_business_type,
                &item.account_role,
                &item.account_status
            ]
        ));
        let mut accounts = try!(self.read(&repository::UserClause::Id(id.into())));
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
    SELECT *, account.id as account_id, account.asset_type as account_asset_type
    FROM account 
    WHERE account.id = $1";

const SELECT_BY_EMAIL: &'static str = "
    SELECT *, account.id as account_id, account.asset_type as account_asset_type
    FROM account 
    JOIN account_owner ON account.owner_id = account_owner.id 
    JOIN users ON account_owner.user_id = users.id
    WHERE
    users.email_address = $1";

const INSERT: &'static str = "INSERT INTO 
    account(owner_id, asset_type, account_type, account_business_type, account_role, account_status) 
    VALUES($1, $2, $3, $4, $5, $6)
    RETURNING id;";

const UPDATE_BY_ID: &'static str = "UPDATE account SET 
    owner_id = $2, 
    asset_type = $3, 
    account_type = $4, 
    account_business_type = $5, 
    account_role = $6, 
    account_status = $7
    WHERE id = $1";
