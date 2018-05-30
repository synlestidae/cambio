use query::{TableName, BoolClause, Column};

pub struct SelectSpec {
    table: TableName,
    columns: Vec<Column>,
    clause: Option<BoolClause>
}

impl SelectSpec {
    pub fn get_sql_query(&self) -> String {
        let clause = match self.clause {
            None => format!(""),
            Some(ref clause) => format!("WHERE {}", clause.get_where_clause())
        };
        format!("SELECT {} FROM {} {}", self.get_columns(), self.table.name(), clause)
    }

    fn get_columns(&self) -> String {
        self.columns.iter()
            .map(|col| col.get_column())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
