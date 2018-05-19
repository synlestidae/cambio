use db::{TryFromRow, TryFromRowError};
use db;
use domain::Id;
use domain;
use postgres;
use postgres::rows::Rows;
use repository;
use repository::*;
use std::path::{Path, PathBuf};
use std::convert::Into;

#[derive(Clone)]
pub struct MediaRepository<T: db::PostgresHelper> {
    db_helper: T,
    base_path: PathBuf,
}

impl<T: db::PostgresHelper> MediaRepository<T> {
    pub fn new(db: T, base_path: &Path) -> Self {
        MediaRepository {
            db_helper: db,
            base_path: base_path.to_path_buf(),
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for MediaRepository<T> {
    type Item = domain::StoredMedia;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        let read_tuple: (&'static str, Vec<&postgres::types::ToSql>) = match clause {
            &repository::UserClause::Id(ref id) => unimplemented!(),
            &repository::UserClause::EmailAddress(ref id) => unimplemented!(),
            bad_clause => {
                return Err(db::CambioError::shouldnt_happen(
                    "Can't find the file you uploaded",
                    &format!("Unsupported clause: {:?}", bad_clause),
                ))
            }
        };
        let (sql, params) = read_tuple;
        let rows: Vec<MediaRow> = try!(self.db_helper.query(sql, &params));
        let mut items = Vec::new();
        for row in rows.into_iter() {
            let item = try!(row.into());
            items.push(item);
        }
        Ok(items)
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for MediaRepository<T> {
    type Item = domain::StoredMedia;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let size = match item.resource.size() {
            Ok(s) => s as u32,
            _ => {
                return Err(db::CambioError::not_found_search(
                    "Could not get file size",
                    "Metadata size() method failed",
                ))
            }
        };
        let rows = try!(self.db_helper.query_raw(
            INSERT,
            &[
                &item.owner_id,
                &item.file_format,
                &item.resource.reference(),
                &size
            ]
        ));
        if rows.len() == 0 {
            Err(db::CambioError::db_update_failed("StoredMedia"))
        } else {
            let row = rows.get(0);
            let id_match: Option<Id> = row.get(0);
            let mut item_vec = match id_match {
                Some(id) => try!(self.read(&repository::UserClause::Id(id))),
                None => {
                    return Err(db::CambioError::shouldnt_happen(
                        "Cannot find ID for stored item",
                        "Failed to get ID",
                    ))
                }
            };
            item_vec
                .pop()
                .ok_or(db::CambioError::db_update_failed("StoredMedia"))
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoUpdate for MediaRepository<T> {
    type Item = domain::StoredMedia;

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let size = match item.resource.size() {
            Ok(s) => s as u32,
            _ => {
                return Err(db::CambioError::not_found_search(
                    "Could not get file size",
                    "Metadata size() method failed",
                ))
            }
        };
        let mut updated: Vec<MediaRow> = try!(self.db_helper.query(
            UPDATE,
            &[
                &item.id,
                &item.owner_id,
                &item.file_format,
                &item.resource.reference(),
                &size
            ]
        ));
        let updated_match: MediaRow = try!(
            updated
                .pop()
                .ok_or(db::CambioError::db_update_failed("StoredMedia"))
        );
        updated_match.into()
    }
}

impl<T: db::PostgresHelper> repository::RepoDelete for MediaRepository<T> {
    type Item = domain::StoredMedia;

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let mut deleted: Vec<MediaRow> = try!(self.db_helper.query(DELETE, &[&item.id]));

        let deleted_match: MediaRow = try!(
            deleted
                .pop()
                .ok_or(db::CambioError::db_update_failed("StoredMedia"))
        );
        deleted_match.into()
    }
}

#[derive(TryFromRow)]
struct MediaRow {
    id: Id,
    owner_id: Option<Id>,
    file_format: domain::MediaFileFormat,
    storage_location: domain::StorageLocation,
    reference: String,
    file_size: i64,
}

impl Into<Result<domain::StoredMedia, db::CambioError>> for MediaRow {
    fn into(self) -> Result<domain::StoredMedia, db::CambioError> {
        let mut unclean_path = PathBuf::new();
        let mut clean_path = PathBuf::new();
        unclean_path.push(&self.reference);
        let file_name = unclean_path.file_name();
        let resource = match file_name {
            Some(p) => {
                clean_path.push(p);
                domain::MediaResource::File(clean_path)
            }
            None => {
                return Err(db::CambioError::shouldnt_happen(
                    "Could not find the file.",
                    "File path is invalid",
                ))
            }
        };
        Ok(domain::StoredMedia {
            id: Some(self.id),
            owner_id: self.owner_id,
            file_format: self.file_format,
            file_size: self.file_size as u64,
            resource: resource,
        })
    }
}

const SELECT_ID: &'static str = "SELECT * FROM media WHERE id = $1";
const SELECT_EMAIL: &'static str = "SELECT * FROM media 
    JOIN users ON media.owner_id = users.id
    WHERE users.email_address = $1";
const SELECT_ALL: &'static str = "
    SELECT * FROM media
";

const INSERT: &'static str = "INSERT INTO 
    media(owner_id, file_format, storage_location, reference, file_size) 
    VALUES ($1, $2, 'webserver_local', $3, $4) 
    RETURNING id";

const UPDATE: &'static str = "UPDATE media
    SET owner_id = $2, file_format = $3, storage_location = $4, reference = $5, file_size = $6
    WHERE id = $1
    RETURNING *";

const DELETE: &'static str = "DELETE media
    WHERE id = $1
    RETURNING *";
