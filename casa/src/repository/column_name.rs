pub struct ColumnName(String);

impl ColumnName {
    pub fn new(name: String) -> Self {
        ColumnName(name)
    }
}
