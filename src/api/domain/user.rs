use serde_derive::*;

pub struct UserObj {
    email_address: String;
    password: String;
    challenge_id: Option<String>;
    challenge_response: Option<String>;
}
