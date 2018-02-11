use repository;
use db;
use domain;

#[derive(Clone)]
pub struct AccountRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> AccountRepository<T> {
    pub fn new(db: T) -> Self {
        AccountRepository {
            db_helper: db
        }
    }

    fn _get_asset_id(&mut self, asset_type: &domain::AssetType, asset_denom: &domain::Denom) 
        -> Result<domain::Id, db::CambioError> {
        let asset_id_vec = 
            try!(self.db_helper.query_raw("SELECT get_asset_type($1, $2) AS id", 
                &[asset_type, asset_denom]));
        if asset_id_vec.len() == 0 {
            return Err(db::CambioError::bad_input("Unknown asset and denom combination", 
                "Asset ID not found"));
        }
        let asset_id_match: Option<domain::Id> = asset_id_vec.get(0).get("id");
        if asset_id_match.is_none() {
            Err(db::CambioError::format_obj("Unknown asset type", "Unknown asset type"))
        } else {
            Ok(asset_id_match.unwrap())
        }
    }
}

impl<T: db::PostgresHelper> repository::Repository for AccountRepository<T> {
    type Item = domain::Account;
    type Clause = repository::AccountClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        match clause {
            &repository::AccountClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::AccountClause::EmailAddress(ref email_address) => self.db_helper.query(SELECT_BY_EMAIL,
                &[email_address])
        }
    }

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let err = db::CambioError::shouldnt_happen(
            "Failed to locate asset account after creating it", 
            "Error during asset account creation");
        let asset_id: domain::Id = try!(self._get_asset_id(&item.asset_type, &item.asset_denom));
        let rows = self.db_helper.query_raw(INSERT, &[
            &item.owner_user_id, 
            &asset_id,
            &item.account_type,
            &item.account_business_type,
            &item.account_role,
            &item.account_status
        ]);
        let result_list = try!(rows);
        if result_list.len() == 0 {
            Err(db::CambioError::shouldnt_happen(
                "Failed to create account", 
                "INSERT query didn't return ID"))
        } else {
            let row = result_list.get(0);
            let id_match: Option<domain::Id> = row.get("id");
            if id_match.is_none() {
                return Err(db::CambioError::shouldnt_happen(
                    "Could not find account after creating it", 
                    "Row did not return ID of account")
                );
            }
            let id = id_match.unwrap();
            let mut accounts = try!(self.read(&repository::AccountClause::Id(id)));
            match accounts.pop() {
                Some(account) => Ok(account),
                None => Err(db::CambioError::shouldnt_happen(
                    "Could not find account after creating it", 
                    "read() after inserting account returned empty list"))
            }
        }
    }

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let (id, owner_id) = match (&item.id, &item.owner_user_id) {
            (&Some(id), &Some(owner_id)) => (id, owner_id),
            _ => return Err(db::CambioError::format_obj("Cannot update without account and owner ID",
                "Account and/or Owner ID was None"))
        };
        let asset_id = try!(self._get_asset_id(&item.asset_type, &item.asset_denom));
        try!(self.db_helper.execute(UPDATE_BY_ID, &[&id,
            &owner_id, 
            &asset_id,
            &item.account_type,
            &item.account_business_type,
            &item.account_role,
            &item.account_status
        ]));
        let mut accounts = try!(self.read(&repository::AccountClause::Id(asset_id)));
        match accounts.pop() {
            Some(account) => Ok(account),
            None => {
                Err(db::CambioError::shouldnt_happen(
                    "Could not locate account after update", 
                    "read(id) after update() returned empty Vec"))
            }
        }
    }

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        Err(db::CambioError::shouldnt_happen(
            "Cannot remove asset account from database", 
            "DELETE for account table not supported"
        ))
    }

    
}

const SELECT_BY_ID: &'static str = "SELECT *, id as account_id FROM account WHERE id = $1";

const SELECT_BY_EMAIL: &'static str = "SELECT *, id as account_id 
    FROM account WHERE id = $1
    JOIN account_owner ON account.owner_id = account_owner.id 
    JOIN users ON account_owner.user_id = users.id";

const INSERT: &'static str = "INSERT INTO 
    account(owner_id, asset_type, account_type, account_business_type, account_role, account_status) 
    VALUES($1, $2, $3, $4, $5, $6)
    RETURNING id;";

const UPDATE_BY_ID: &'static str = "UPDATE account SET 
    owner_id = $2, asset_type = $3, account_type = $4, account_business_type = $5, account_role = $6, account_status = $7
    WHERE id = $1";