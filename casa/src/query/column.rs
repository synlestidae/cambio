use query::Field;

pub enum Column {
    Column(Field),
    ColumnAs(Field, Field),
    All
}

impl Column {
    pub fn get_column(&self) -> String {
        match self {
            &Column::Column(ref col) => format!("{}", col.name()),
            &Column::ColumnAs(ref col, ref name) => format!("{} AS {}", col.name(), name.name()),
            &Column::All => "*".to_owned(),
        }
    }
}
