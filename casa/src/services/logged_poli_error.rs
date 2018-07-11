use services::PoliError;
use domain::UserId;

pub struct LoggedPoliError(UserId, PoliError);
