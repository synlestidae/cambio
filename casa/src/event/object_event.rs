use event::EventKey;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use colectivo::message::Message;

/*#[derive(Serialize, Deserialize)]
pub struct ObjectEvent<E: EventKey + Serialize + DeserializeOwned, T: Serialize + DeserializeOwned>  {
    object: E,
    event_type: T
}

impl<'a, E: EventKey + Serialize + DeserializeOwned, T: Serialize + DeserializeOwned> 
    Into<Message> for ObjectEvent<E, T> {
    fn into(self) -> Message {
        let key = self.object.key();
        let bytes = serde_json::to_string(&self).unwrap().into_bytes();
        Message::new(key, bytes)
    }
}*/
