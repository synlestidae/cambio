use domain::{Id, MediaFileFormat, MediaResource};

pub struct StoredMedia {
    id: Option<Id>,
    owner_id: Id,
    file_format: MediaFileFormat,
    resource: MediaResource
}
