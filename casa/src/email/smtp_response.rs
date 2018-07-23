use lettre::smtp::response::Code;

pub struct SMTPResponse {
    was_successful: bool
}

impl SMTPResponse {
    fn from_code(code: &Code) -> Self {
        unimplemented!()
    }
}
