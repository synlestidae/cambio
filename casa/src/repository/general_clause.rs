use repository;
use postgres::types::ToSql;

pub struct GeneralClause(pub Vec<(repository::ColumnName, String)>);

impl repository::Clause for GeneralClause {
    fn get_clause<'a>(&'a self) -> Vec<(repository::ColumnName, String)> {
        self.0.iter().map(|&(ref c, ref s)| (c.clone(), s.clone())).collect()
    }
}
