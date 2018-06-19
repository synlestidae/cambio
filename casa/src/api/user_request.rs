use api;

#[derive(Debug)]
pub enum UserRequest {
    Register(api::Registration), 
    Confirm(api::RegistrationConfirm), 
    LogIn(api::LogIn), 
}
