use db::{get_value, get_value_option};
use db::{TryFromRow, TryFromRowError};
use domain::{Id, UserId, PhotoStatus};
use postgres;
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TryFromRow)]
pub struct PersonalIdentity {
    #[column_id(personal_identity_id)]
    pub id: Option<Id>,
    pub user_id: UserId,
    pub nz_passport_number: Option<String>,
    pub nz_drivers_licence_number: Option<String>,

    pub face_with_document_photo: Option<Id>,
    pub face_photo: Option<Id>,
    pub document_scan_photo: Option<Id>,

    // status of each photo
    pub face_with_document_status: Option<PhotoStatus>,
    pub face_status: Option<PhotoStatus>,
    pub document_scan_status: Option<PhotoStatus>,
}

/*impl TryFromRow for PersonalIdentity {
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
}*/
