use colectivo::message_key::MessageKey;

pub trait EventKey {
    fn key(&self) -> MessageKey;
}
