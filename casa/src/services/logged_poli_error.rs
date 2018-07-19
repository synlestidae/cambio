use domain::UserId;
use services::PoliError;

pub struct LoggedPoliError(UserId, PoliError);
