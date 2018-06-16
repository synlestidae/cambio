use api;

#[derive(Debug)]
pub enum UserRequest {
    Register(api::Registration), 
    LogIn(api::LogIn), 
}
