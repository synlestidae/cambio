use std::sync::mpsc::SendError;
use colectivo::ProducerError;
use serde_json::Error as SerdeError;

pub enum BusSendError {
    SendError,
    SerdeError
}

impl<T> From<SendError<T>> for BusSendError {
    fn from(err: SendError<T>) -> Self {
        BusSendError::SendError
    }
}

impl From<ProducerError> for BusSendError {
    fn from(err: ProducerError) -> Self {
        BusSendError::SendError
    }
}

impl From<SerdeError> for BusSendError {
    fn from(err: SerdeError) -> Self {
        BusSendError::SerdeError
    }
}
