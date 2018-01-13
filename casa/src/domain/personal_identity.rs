use domain::{Id, PhotoStatus};
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;
use db::{get_value, get_value_option};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl TryFromRow for PersonalIdentity {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let id: Option<Id> = try!(get_value_option(
            "PersonalIdentity",
            "personal_identity_id",
            row,
        ));
        let nz_passport_number: Option<String> = try!(get_value_option(
            "PersonalIdentity",
            "nz_passport_number",
            row,
        ));
        let nz_drivers_licence_number: Option<String> = try!(get_value_option(
            "PersonalIdentity",
            "nz_drivers_licence_number",
            row,
        ));

        let face_with_document_photo: Option<Id> = try!(get_value_option(
            "PersonalIdentity",
            "face_with_document_photo",
            row,
        ));
        let face_photo: Option<Id> = try!(get_value_option("PersonalIdentity", "face_photo", row));
        let document_scan_photo: Option<Id> = try!(get_value_option(
            "PersonalIdentity",
            "document_scan_photo",
            row,
        ));

        let face_with_document_status: Option<PhotoStatus> = try!(get_value_option(
            "PersonalIdentity",
            "face_with_document_status",
            row,
        ));
        let face_status: Option<PhotoStatus> =
            try!(get_value_option("PersonalIdentity", "face_status", row));
        let document_scan_status: Option<PhotoStatus> = try!(get_value_option(
            "PersonalIdentity",
            "document_scan_status",
            row,
        ));

        Ok(PersonalIdentity {
            id: id,
            nz_passport_number: nz_passport_number,
            nz_drivers_licence_number: nz_drivers_licence_number,

            face_with_document_photo: face_with_document_photo,
            face_photo: face_photo,
            document_scan_photo: document_scan_photo,

            // status of each photo
            face_with_document_status: face_with_document_status,
            face_status: face_status,
            document_scan_status: document_scan_status,
        })
    }
}
