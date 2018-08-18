use colectivo::MessageKey;

pub trait EventKey {
    fn key(&self) -> MessageKey;
}
