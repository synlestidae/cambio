use colectivo::Topic;

pub trait EventTopic {
    fn topic() -> Topic;
}
