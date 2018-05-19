use postgres::rows::Row;
use postgres::types::FromSql;
use db::TryFromRowError;
use postgres;

pub fn get_value<'a, T>(entity: &str, field: &str, row: &Row<'a>) -> Result<T, TryFromRowError>
where
    T: FromSql,
{
    match get_value_option(entity, field, row) {
        Ok(None) => Err(TryFromRowError::missing_field(entity, field)),
        Ok(Some(string)) => Ok(string),
        Err(error) => Err(error),
    }
}

pub fn get_value_option<'a, T>(
    entity: &str,
    field: &str,
    row: &Row<'a>,
) -> Result<Option<T>, TryFromRowError>
where
    T: FromSql,
{
    let get_result: Option<postgres::Result<Option<T>>> = row.get_opt(field);
    match get_result {
        Some(Ok(value)) => Ok(value),
        Some(Err(err)) => Err(TryFromRowError::new(&format!(
            "Error getting field '{}' for {}: {}",
            field, entity, err
        ))),
        None => Err(TryFromRowError::missing_field(entity, field)),
    }
}
