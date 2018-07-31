use api;

#[derive(Debug)]
pub enum UserRequest {
    Register(api::Registration),
    ResendEmail(api::ResendEmail),
    Confirm(api::RegistrationConfirm),
    LogIn(api::LogIn),
    SetPersonalDetails(api::PersonalDetails),
    GetPersonalDetails
}
