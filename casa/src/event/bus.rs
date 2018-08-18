use serde::{Serialize};
use serde::de::DeserializeOwned;
use colectivo::Producer;
use colectivo::Consumer;
use colectivo::Message;
use event::*;
use serde_json;

pub struct Bus {
    producer: Producer,
    consumer: Consumer
}

impl Bus {
    pub fn new(producer: Producer, consumer: Consumer) -> Self {
        Self {
            producer: producer,
            consumer: consumer
        }
    }

    pub fn send<E: EventKey + Serialize + DeserializeOwned, T: Serialize + DeserializeOwned>(&self, obj: &E, ty: &T) -> Result<(), BusSendError> {
        let bytes = serde_json::to_string(&(ty, obj))?;
        let message = Message::new(obj.key(), bytes);
        self.producer.send(message)?;
        Ok(())
    }

    pub fn try_recv<E: EventKey + Serialize + DeserializeOwned, T: Serialize + DeserializeOwned>(&self) -> Result<(E, T), BusRecvError> {
        let event = self.consumer.try_recv()?;
        let result = serde_json::from_slice(&event.payload.0)?;
        Ok(result)
    }

    pub fn recv<E: EventKey + Serialize + DeserializeOwned, T: Serialize + DeserializeOwned>(&self) -> Result<(E, T), BusRecvError> {
        let event = self.consumer.recv()?;
        let result = serde_json::from_slice(&event.payload.0)?;
        Ok(result)
    }
}
