use db::{PostgresHelper, CambioError};
use domain::{Address, PersonalIdentity, ContactInfo, Id};

pub struct ProfileService<T: PostgresHelper> {
    db_helper: T,
}

impl<T: PostgresHelper> ProfileService<T> {
    pub fn new(db_helper: T) -> Self {
        ProfileService { db_helper: db_helper }
    }

    pub fn get_user_address(
        &mut self,
        email_address: &str,
    ) -> Result<Option<Address>, CambioError> {
        unimplemented!()
    }

    pub fn set_user_address(
        &mut self,
        email_address: &str,
        address: &Address,
    ) -> Result<Address, CambioError> {
        unimplemented!()
    }

    pub fn get_contact_info(
        &mut self,
        email_address: &str,
    ) -> Result<Option<ContactInfo>, CambioError> {
        unimplemented!();
    }

    pub fn set_contact_info(
        &mut self,
        email_address: &str,
        info: &ContactInfo,
    ) -> Result<ContactInfo, CambioError> {
        unimplemented!();
    }

    pub fn get_personal_info(
        &mut self,
        email_address: &str,
    ) -> Result<Option<PersonalIdentity>, CambioError> {
        unimplemented!();
    }

    pub fn set_personal_info(
        &mut self,
        email_address: &str,
        info: &PersonalIdentity,
    ) -> Result<Option<PersonalIdentity>, CambioError> {
        unimplemented!();
    }
}
