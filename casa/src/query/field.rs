pub struct Field(String);

impl Field {
    pub fn parse(name: &str) -> Option<Self> {
        Some(Field(name.to_owned()))
    }

    pub fn name(&self) -> String {
        self.0.clone()
    }
}
