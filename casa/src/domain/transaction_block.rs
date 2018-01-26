pub struct EthereumBlock {
    id: Option<Id>,
    time: DateTime<Utc>, 
    block: u64,
    block_hash: String 
}
