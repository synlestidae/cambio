pub struct ObjectEvent<E: EventKey + Serialize + Deserialize, T> {
    event: E,
    event_type
}
