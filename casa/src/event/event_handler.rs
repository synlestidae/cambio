use serde::de::DeserializeOwned;
use event::EventKey;

pub trait EventHandler {
    type E: EventKey + DeserializeOwned;
    type Ty: DeserializeOwned;

    fn handle(e: Self::E, ty: Self::Ty);
}
