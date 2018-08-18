use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::RecvError;
use colectivo::consumer_error::ConsumerError;
use colectivo::consumer_try_error::ConsumerTryError;
use serde_json::Error as SerdeError;

pub enum BusRecvError {
    TryRecvError(TryRecvError),
    RecvError(RecvError),
    ConsumerError,
    SerdeError
}

impl From<TryRecvError> for BusRecvError {
    fn from(err: TryRecvError) -> Self {
        BusRecvError::TryRecvError(err)
    }
}

impl From<RecvError> for BusRecvError {
    fn from(err: RecvError) -> Self {
        BusRecvError::RecvError(err)
    }
}

impl From<ConsumerError> for BusRecvError {
    fn from(err: ConsumerError) -> Self {
        BusRecvError::ConsumerError
    }
}

impl From<ConsumerTryError> for BusRecvError {
    fn from(err: ConsumerTryError) -> Self {
        BusRecvError::ConsumerError
    }
}

impl From<SerdeError> for BusRecvError {
    fn from(err: SerdeError) -> Self {
        BusRecvError::SerdeError
    }
}
