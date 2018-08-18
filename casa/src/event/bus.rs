use serde::{Serialize};
use serde::de::DeserializeOwned;
use colectivo::Colectivo;
use colectivo::Producer;
use colectivo::Consumer;
use colectivo::Message;
use colectivo::Topic;
use event::*;
use serde_json;

#[derive(Clone)]
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

    pub fn from_colectivo<T: Into<Topic>>(t: T, c: &mut Colectivo) -> Self {
        let topic = t.into();
        Self {
            producer: c.producer(topic.clone()),
            consumer: c.consumer(topic)
        }
    }

    pub fn from_topic<T: Into<Topic>>(t: T) -> Self {
        let colectivo = Colectivo::new();
        let topic = t.into();
        Self {
            producer: colectivo.producer(topic.clone()),
            consumer: colectivo.consumer(topic)
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
