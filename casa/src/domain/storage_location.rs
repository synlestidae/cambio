#[derive(Debug, Eq, PartialEq, ToSql, FromSql)]
#[postgres(name = "storage_location_type")]
pub enum StorageLocation {
    #[postgres(name = "webserver_local")]
    WebserverLocal
}
