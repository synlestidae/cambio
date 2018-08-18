use serde::de::DeserializeOwned;
use event::EventKey;

pub trait EventHandler {
    type E: DeserializeOwned;
    type Ty: DeserializeOwned;

    fn handle(&mut self, e: Self::E, ty: Self::Ty);
}
