use domain::{AssetType, Denom, Id, AccountStatus, AccountBusinessType, AccountRole};
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: Option<Id>,
    pub owner_user_id: Option<Id>,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub account_status: AccountStatus,
    pub account_business_type: AccountBusinessType,
    pub account_role: AccountRole,
}

impl TryFromRow for Account {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let account_id: Option<Id> = row.get("account_id");
        let account_user_id_match: Option<Id> = row.get("user_id");
        let asset_type = try!(AssetType::try_from_row(row));
        let denom = try!(Denom::try_from_row(row));

        let account_status_match: Option<AccountStatus> = row.get("account_status");
        let account_status = try!(
            account_status_match.ok_or(TryFromRowError::missing_field("Account", "account_status"))
        );

        let account_role_match: Option<AccountRole> = row.get("account_role");
        let account_role = try!(
            account_role_match.ok_or(TryFromRowError::missing_field("Account", "account_role"))
        );

        let account_business_type_match: Option<AccountBusinessType> =
            row.get("account_business_type");
        let account_business_type = try!(
            account_business_type_match.ok_or(TryFromRowError::missing_field("Account",
                "account_business_type"))
        );


        Ok(Account {
            id: account_id,
            owner_user_id: account_user_id_match,
            asset_type: asset_type,
            asset_denom: denom,
            account_status: account_status,
            account_role: account_role,
            account_business_type: account_business_type,
        })
    }
}
