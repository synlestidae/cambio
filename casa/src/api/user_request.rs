use api;

pub enum UserRequest {
    Register(api::Registration), 
    LogIn(api::LogIn), 
}
