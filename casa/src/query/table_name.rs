pub struct TableName(String);

impl TableName {
    pub fn parse(name: &str) -> Option<Self> {
        Some(TableName(name.to_owned()))
    }

    pub fn name(&self) -> String {
        self.0.to_string()
    }
}
