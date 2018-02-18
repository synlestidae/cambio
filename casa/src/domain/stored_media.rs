use domain::{Id, MediaFileFormat, MediaResource};
use std;

pub struct StoredMedia {
    pub id: Option<Id>,
    pub owner_id: Option<Id>,
    pub file_format: MediaFileFormat,
    pub file_size: u64,
    pub resource: MediaResource
}
