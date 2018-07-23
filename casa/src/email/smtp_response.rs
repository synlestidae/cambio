use lettre::smtp::response::Code;
use lettre::smtp::response::Severity;

#[derive(Debug)]
pub struct SMTPResponse {
    was_successful: bool
}

impl SMTPResponse {
    fn from_code(code: &Code) -> Self {
        Self {
            was_successful: code.severity == Severity::PositiveCompletion
        }
    }
}
