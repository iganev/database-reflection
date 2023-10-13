use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

#[allow(dead_code)]
const METADATA_CHARSET: &str = "charset";
#[allow(dead_code)]
const METADATA_COLLATION: &str = "collation";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
pub enum DefaultValue {
    #[default]
    Null,
    Value(Value)
}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database<'n> {
    name: &'n str,
    #[serde_as(as = "Vec<(_, _)>")]
    tables: HashMap<String, Table<'n>>,
    #[serde_as(as = "Vec<(_, _)>")]
    constraints: HashMap<String, Constraint<'n>>,
    metadata: HashMap<String, String>
}

impl<'n> Database<'n> {
    pub fn new(name: &'n str) -> Database {
        Database {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Database<'n> {
        self.metadata.insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

    pub fn set_table(&mut self, table: Table<'n>) -> &mut Database<'n> {
        self.tables.insert(table.name.to_string(), table);

        self
    }

    pub fn table(&self, key: &str) -> Option<&Table> {
        self.tables.get(key)
    }

}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table<'n> {
    name: &'n str,
    #[serde_as(as = "Vec<(_, _)>")]
    columns: HashMap<String, Column<'n>>,
    constraints: HashMap<String, Constraint<'n>>,
    indexes: HashMap<String, Index<'n>>,
    metadata: HashMap<String, String>
}

impl<'n> Table<'n> {
    pub fn new(name: &'n str) -> Table {
        Table {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column<'n> {
    database: &'n str,
    table: &'n str,
    name: &'n str,
    datatype: Datatype,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultValue>,
}

impl<'n> Column<'n> {
    pub fn new(database: &'n str, table: &'n str, name: &'n str, datatype: Datatype) -> Column<'n> {
        Column {
            database,
            table,
            name,
            datatype,
            ..Default::default()
        }
    }

    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column<'n> {
        self.default = value;
        self
    }

    pub fn database(&self) -> &str {
        self.database
    }

    pub fn table(&self) -> &str {
        self.table
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index<'n> {
    name: &'n str,
    column: Column<'n>,
    unique: bool
}

impl<'n> Index<'n> {
    pub fn new(name: &'n str, column: Column<'n>, unique: bool) -> Self {
        Index {
            name,
            column,
            unique
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn column(&self) -> &Column<'n> {
        &self.column
    }

    pub fn unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Constraint<'n> {
    name: &'n str,
    local: Column<'n>,
    foreign: Column<'n>,
    metadata: HashMap<String, String>
}

impl<'n> Constraint<'n> {
    pub fn new(name: &'n str, local: Column<'n>, foreign: Column<'n>) -> Self {
        Constraint {
            name,
            local,
            foreign,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn local(&self) -> &Column {
        &self.local
    }

    pub fn foreign(&self) -> &Column {
        &self.foreign
    }

    pub fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Constraint<'n> {
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
        let index = Index::new("index_1", test_id.clone(), false);

        let fk_test_id = Column::new(db.name(), "children", "test_id", Datatype::Int(10));

        let fk = Constraint::new("fk_1", test_id.clone(), fk_test_id);

        db.set_table(table);

        assert!(db.table("test").is_some());
        assert_eq!(db.table("test").unwrap().name, "test");

        // assert_eq!(index.name(), "index_1");
        // assert_eq!(index.column(), &test_id.clone());

        println!("{}", serde_json::to_string(&db).unwrap());



    }
}