use query::{Field, Value};

pub enum BoolClause {
    And(Box<BoolClause>, Box<BoolClause>),
    Or(Box<BoolClause>, Box<BoolClause>),
    Not(Box<BoolClause>),
    Equals(Field, Value),
}

impl BoolClause {
    pub fn get_where_clause(&self) -> String {
        match self {
            &BoolClause::And(ref l, ref r) => {
                format!("{} AND {}", l.get_where_clause(), r.get_where_clause())
            }
            &BoolClause::Or(ref l, ref r) => {
                format!("{} OR {}", l.get_where_clause(), r.get_where_clause())
            }
            &BoolClause::Not(ref expr) => format!("NOT ({})", expr.get_where_clause()),
            &BoolClause::Equals(ref field, ref value) => {
                format!("{} = {}", field.name(), value.to_sql_value())
            }
        }
    }

    pub fn id_equals(val: i32) -> Self {
        let id = Field::parse("id").unwrap();
        BoolClause::Equals(id, Value::Signed32(val))
    }
}
