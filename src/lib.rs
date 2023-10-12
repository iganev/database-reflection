use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

#[allow(dead_code)]
const METADATA_CHARSET: &str = "charset";
#[allow(dead_code)]
const METADATA_COLLATION: &str = "collation";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
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

    Enum(Vec<String>),
    Set(Vec<String>),
    Json(String)
}

impl Default for Datatype {
    fn default() -> Self {
        Datatype::Varchar(45)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum DefaultValue {
    #[default]
    Null,
    Value(Value)
}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database {
    name: String,
    #[serde_as(as = "Vec<(_, _)>")]
    tables: HashMap<String, Table>,
    #[serde_as(as = "Vec<(_, _)>")]
    constraints: HashMap<String, Constraint>,
    metadata: HashMap<String, String>
}

impl Database {
    pub fn new(name: impl ToString) -> Database {
        Database {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Database {
        self.metadata.insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

    pub fn set_table(&mut self, table: Table) -> &mut Database {
        self.tables.insert(table.name.clone(), table);

        self
    }

    pub fn table(&self, key: &str) -> Option<&Table> {
        self.tables.get(key)
    }

}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table {
    name: String,
    #[serde_as(as = "Vec<(_, _)>")]
    columns: HashMap<String, Column>,
    constraints: HashMap<String, Constraint>,
    indexes: HashMap<String, Index>,
    metadata: HashMap<String, String>
}

impl Table {
    pub fn new(name: impl ToString) -> Table {
        Table {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    database: String,
    table: String,
    name: String,
    datatype: Datatype,
    default: Option<DefaultValue>,
}

impl Column {
    pub fn new(database: impl ToString, table: impl ToString, name: impl ToString, datatype: Datatype) -> Column {
        Column {
            database: database.to_string(),
            table: table.to_string(),
            name: name.to_string(),
            datatype,
            ..Default::default()
        }
    }

    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column{
        self.default = value;
        self
    }

    pub fn database(&self) -> &str {
        self.database.as_str()
    }

    pub fn table(&self) -> &str {
        self.table.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }

}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Index {
    name: String,
    column: Column,
    unique: bool
}

impl Index {
    pub fn new(name: impl ToString, column: Column, unique: bool) -> Self {
        Index {
            name: name.to_string(),
            column,
            unique
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn column(&self) -> &Column {
        &self.column
    }

    pub fn unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Constraint {
    name: String,
    local: Column,
    foreign: Column,
    metadata: HashMap<String, String>
}

impl Constraint {
    pub fn new(name: impl ToString, local: Column, foreign: Column) -> Constraint {
        Constraint {
            name: name.to_string(),
            local,
            foreign,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn local(&self) -> &Column {
        &self.local
    }

    pub fn foreign(&self) -> &Column {
        &self.local
    }

    pub fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Constraint {
        self.metadata.insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let db_name = "test";

        let mut db = Database::new(db_name);

        db.set_meta(METADATA_CHARSET, "utf8mb4").set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        assert_eq!(db.name(), db_name);
        assert_eq!(db.meta(METADATA_CHARSET), Some(String::from("utf8mb4")));
        assert_eq!(db.meta(METADATA_COLLATION), Some(String::from("utf8mb4_unicode_ci")));
        assert_eq!(db.meta("shit"), None);

        let table = Table::new("test");

        let test_id = Column::new(db.name(), table.name(), "test_id", Datatype::Int(10));

        let fk_test_id = Column::new(db.name(), "children", "test_id", Datatype::Int(10));

        db.set_table(table);

        assert!(db.table("test").is_some());
        assert_eq!(db.table("test").unwrap().name, "test");

        let index = Index::new("index_1", test_id.clone(), false);

        assert_eq!(index.name(), "index_1");
        assert_eq!(index.column(), &test_id);

        println!("{}", serde_json::to_string(&db).unwrap());

        let fk = Constraint::new("fk_1", test_id, fk_test_id);

    }
}