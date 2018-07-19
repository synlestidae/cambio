use query::{BoolClause, Column, Field, TableName};

pub struct SelectSpec {
    pub table: TableName,
    pub columns: Vec<Column>,
    pub clause: Option<BoolClause>,
}

impl SelectSpec {
    pub fn get_sql_query(&self) -> String {
        let clause = match self.clause {
            None => format!(""),
            Some(ref clause) => format!("WHERE {}", clause.get_where_clause()),
        };
        format!(
            "SELECT {} FROM {} {}",
            self.get_columns(),
            self.table.name(),
            clause
        )
    }

    fn get_columns(&self) -> String {
        self.columns
            .iter()
            .map(|col| col.get_column())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /*pub fn join_on(self, table: &TableName, this_field: &Field, other_field: &Field) -> SelectSpec {
        unimplemented!()
    }

    pub fn select(self, column: Column) -> SelectSpec {
        unimplemented!()
    }*/

    pub fn join_spec(self, spec: SelectSpec, on: Vec<(Field, Field)>) -> SelectSpec {
        unimplemented!()
    }
}
