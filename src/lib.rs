use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
const METADATA_CHARSET: &str = "charset";
#[allow(dead_code)]
const METADATA_COLLATION: &str = "collation";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Datatype<'a> {
    Tinyint(u32),
    Int(u32),
    Bigint(u32),
    Float(u32, u32),
    Real(u32, u32),

    Date,
    Time,
    Datetime,
    Timestamp,

    Char(u32),
    Varchar(u32),
    Text(u32),

    Binary(u32),
    Varbinary(u32),

    Enum(Vec<&'a str>),
    Set(Vec<&'a str>),
    Json(&'a str)
}

impl Default for Datatype<'_> {
    fn default() -> Self {
        Datatype::Varchar(45)
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum DefaultValue {
    #[default]
    Null,
    Value(Value)
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database<'a> {
    name: String,
    #[serde(borrow)]
    tables: HashMap<&'a str, Table<'a>>,
    #[serde(borrow)]
    constraints: HashMap<&'a str, Constraint<'a>>,
    metadata: HashMap<String, String>
}

impl <'a>Database<'a> {
    pub fn new(name: impl ToString) -> Database<'a> {
        Database {
            name: name.to_string(),
            tables: Default::default(),
            constraints: Default::default(),
            metadata: Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_meta(mut self, meta_key: impl ToString, meta_value: impl ToString) -> Database<'a> {
        self.metadata.insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }



}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table<'a> {
    name: &'a str,
    columns: HashMap<&'a str, Column<'a>>,
    constraints: HashMap<&'a str, Constraint<'a>>,
    indexes: HashMap<&'a str, Index<'a>>,
    metadata: HashMap<String, String>
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Column<'a> {
    database: &'a str,
    table: &'a str,
    name: &'a str,
    datatype: Datatype<'a>,
    default: Option<DefaultValue>,
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Index<'a> {
    name: &'a str,
    column: &'a str
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constraint<'a> {
    name: &'a str,
    column: &'a str,
    #[serde(borrow)]
    foreign: Column<'a>,
    metadata: HashMap<String, String>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let db_name = "test";

        let db = Database::new(db_name).set_meta(METADATA_CHARSET, "utf8mb4").set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        assert_eq!(db.name(), db_name);
        assert_eq!(db.meta(METADATA_CHARSET), Some(&String::from("utf8mb4")));
        assert_eq!(db.meta(METADATA_COLLATION), Some(&String::from("utf8mb4_unicode_ci")));
        assert_eq!(db.meta("shit"), None);

        let table = Table::default();

    }
}