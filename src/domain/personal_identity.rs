use domain::{Id, PhotoStatus};

pub struct PersonalIdentity {
    id: Option<Id>,
    nz_passport_number: Option<String>,
    nz_drivers_licence_number: Option<String>,

    face_with_document_photo: Option<Id>,
    face_photo: Option<Id>,
    document_scan_photo: Option<Id>,

    // status of each photo
    face_with_document_status: Option<PhotoStatus>,
    face_status: Option<PhotoStatus>,
    document_scan_status: Option<PhotoStatus>,
}
