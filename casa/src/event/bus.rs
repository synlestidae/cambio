use serde::{Deserialize, Serialize};
use colectivo::{Producer, Consumer};
use bus::{BusSendError, BusRecvError};
use serde_json;

pub struct Bus {
    producer: Producer,
    consumer: Consumer
}

impl<'a> Bus {
    type E: EventKey + Deserialize<'a> + Serialize<'a>;

    pub fn new(producer: Producer, consumer: Consumer) -> Self {
        Self {
            producer: producer,
            consumer: consumer
        }
    }

    pub fn send(&self, event: E) -> Result<(), BusSendError> {
        let bytes = serde_json::to_bytes(event);
        let message = Message::new(event.key(), Payload(bytes));
        self.producer.send(message)?;
        Ok(())
    }

    pub fn try_recv(&self) -> Result<E, BusRecvError> {
        let event = self.consumer.try_recv()?;
        let result = serde_json::from_bytes(&event.payload.0)?;
        Ok(event)
    }

    pub fn recv(&self) -> Result<E, BusRecvError> {
        let event = self.consumer.recv()?;
        Ok(event)
    }
}
