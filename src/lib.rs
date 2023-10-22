use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;
use std::collections::HashMap;

#[allow(dead_code)]
const METADATA_CHARSET: &str = "charset";
#[allow(dead_code)]
const METADATA_COLLATION: &str = "collation";
#[allow(dead_code)]
const METADATA_UNSIGNED: &str = "unsigned";
#[allow(dead_code)]
const METADATA_NULLABLE: &str = "nullable";
#[allow(dead_code)]
const METADATA_ON_UPDATE: &str = "on_update";

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
    Json(String),
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
    Value(Value),
}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database<'n> {
    name: &'n str,
    #[serde_as(as = "Vec<(_, _)>")]
    tables: HashMap<String, Table<'n>>,
    #[serde_as(as = "Vec<(_, _)>")]
    constraints: HashMap<String, Constraint<'n>>,
    metadata: HashMap<String, String>,
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

    pub fn set_meta(
        &mut self,
        meta_key: impl ToString,
        meta_value: impl ToString,
    ) -> &mut Database<'n> {
        self.metadata
            .insert(meta_key.to_string(), meta_value.to_string());

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
    metadata: HashMap<String, String>,
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

    pub fn set_column(&mut self, column: Column<'n>) -> &mut Table<'n> {
        self.columns.insert(column.name.to_string(), column);

        self
    }

    pub fn column(&self, key: &str) -> Option<&Column> {
        self.columns.get(key)
    }

    pub fn set_constraint(&mut self, constraint: Constraint<'n>) -> &mut Table<'n> {
        self.constraints
            .insert(constraint.name.to_string(), constraint);

        self
    }

    pub fn constraint(&self, key: &str) -> Option<&Constraint> {
        self.constraints.get(key)
    }

    pub fn set_index(&mut self, index: Index<'n>) -> &mut Table<'n> {
        self.indexes.insert(index.name.to_string(), index);

        self
    }

    pub fn index(&self, key: &str) -> Option<&Index> {
        self.indexes.get(key)
    }

    pub fn set_meta(
        &mut self,
        meta_key: impl ToString,
        meta_value: impl ToString,
    ) -> &mut Table<'n> {
        self.metadata
            .insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
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
    metadata: HashMap<String, String>,
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

    pub fn set_meta(
        &mut self,
        meta_key: impl ToString,
        meta_value: impl ToString,
    ) -> &mut Column<'n> {
        self.metadata
            .insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    pub fn meta(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index<'n> {
    name: &'n str,
    column: &'n str,
    unique: bool,
}

impl<'n> Index<'n> {
    pub fn new(name: &'n str, column: &'n str, unique: bool) -> Self {
        Index {
            name,
            column,
            unique,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn column(&self) -> &str {
        self.column
    }

    pub fn unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Constraint<'n> {
    name: &'n str,
    local: &'n str,
    foreign_table: &'n str,
    foreign_column: &'n str,
    metadata: HashMap<String, String>,
}

impl<'n> Constraint<'n> {
    pub fn new(name: &'n str, local: &'n str, foreign: (&'n str, &'n str)) -> Self {
        Constraint {
            name,
            local,
            foreign_table: foreign.0,
            foreign_column: foreign.1,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn local(&self) -> &str {
        self.local
    }

    pub fn foreign(&self) -> (&str, &str) {
        (self.foreign_table, self.foreign_column)
    }

    pub fn set_meta(
        &mut self,
        meta_key: impl ToString,
        meta_value: impl ToString,
    ) -> &mut Constraint<'n> {
        self.metadata
            .insert(meta_key.to_string(), meta_value.to_string());

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

        // CREATE TABLE `clients` (
        // `client_id` int(10) UNSIGNED NOT NULL,
        // `email` varchar(255) NOT NULL,
        // `password` varchar(64) NOT NULL,
        // `phone` varchar(45) DEFAULT NULL,
        // `first_name` varchar(45) DEFAULT NULL,
        // `last_name` varchar(45) DEFAULT NULL,
        // `is_email_verified` tinyint(1) UNSIGNED NOT NULL DEFAULT 0,
        // `email_verification_code` varchar(64) DEFAULT NULL,
        // `password_reset_code` varchar(64) DEFAULT NULL,
        // `last_access` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
        // `created` timestamp NOT NULL DEFAULT current_timestamp()
        // ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        //
        // ALTER TABLE `clients`
        // ADD PRIMARY KEY (`client_id`),
        // ADD UNIQUE KEY `email_UNIQUE` (`email`);
        //
        // CREATE TABLE `client_tokens` (
        // `client_token_id` int(10) UNSIGNED NOT NULL,
        // `client_id` int(10) UNSIGNED NOT NULL,
        // `auth_token` varchar(64) NOT NULL,
        // `auth_token_expiration_date` timestamp NOT NULL,
        // `remote_address` varchar(64) DEFAULT NULL,
        // `user_agent` varchar(255) DEFAULT NULL,
        // `last_access` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
        // `created` timestamp NOT NULL DEFAULT current_timestamp()
        // ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        //
        // ALTER TABLE `client_tokens`
        // ADD PRIMARY KEY (`client_token_id`),
        // ADD KEY `fk_client_tokens_1_idx` (`client_id`);
        //
        // ALTER TABLE `client_tokens`
        // ADD CONSTRAINT `fk_client_tokens_1` FOREIGN KEY (`client_id`) REFERENCES `clients` (`client_id`) ON DELETE CASCADE ON UPDATE CASCADE;
        //
        // CREATE TABLE `products` (
        // `product_id` int(10) UNSIGNED NOT NULL,
        // `name` varchar(255) DEFAULT NULL,
        // `is_enabled` tinyint(1) UNSIGNED NOT NULL DEFAULT 1
        // ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        //
        // ALTER TABLE `products`
        // ADD PRIMARY KEY (`product_id`);
        //
        // CREATE TABLE `client_products` (
        // `client_product_id` int(10) UNSIGNED NOT NULL,
        // `client_id` int(10) UNSIGNED NOT NULL,
        // `product_id` int(10) UNSIGNED NOT NULL
        // ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        //
        // ALTER TABLE `client_products`
        // ADD PRIMARY KEY (`client_product_id`),
        // ADD KEY `fk_client_products_1_idx` (`client_id`),
        // ADD KEY `fk_client_products_2_idx` (`product_id`);
        //
        // ALTER TABLE `client_products`
        // ADD CONSTRAINT `fk_client_products_1` FOREIGN KEY (`client_id`) REFERENCES `clients` (`client_id`) ON DELETE CASCADE ON UPDATE CASCADE,
        // ADD CONSTRAINT `fk_client_products_2` FOREIGN KEY (`product_id`) REFERENCES `products` (`product_id`) ON DELETE CASCADE ON UPDATE CASCADE;

        let db_name = "test";
        let mut db = Database::new(db_name);

        db.set_meta(METADATA_CHARSET, "utf8mb4")
            .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        let clients_table_name = "clients";
        let mut clients_table = Table::new(clients_table_name);
        clients_table
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "client_id",
                Datatype::Int(10),
            ).set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "email",
                Datatype::Varchar(255),
            ))
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "password",
                Datatype::Varchar(64),
            ))
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "phone",
                Datatype::Varchar(45),
            ).set_meta(METADATA_NULLABLE, METADATA_NULLABLE).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "first_name",
                Datatype::Varchar(45),
            ).set_meta(METADATA_NULLABLE, METADATA_NULLABLE).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "last_name",
                Datatype::Varchar(45),
            ).set_meta(METADATA_NULLABLE, METADATA_NULLABLE).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "is_email_verified",
                Datatype::Tinyint(1),
            ).set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED).set_default(Some(DefaultValue::Value(serde_json::Value::from(0)))).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "email_verification_code",
                Datatype::Varchar(64),
            ).set_meta(METADATA_NULLABLE, METADATA_NULLABLE).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "password_reset_code",
                Datatype::Varchar(64),
            ).set_meta(METADATA_NULLABLE, METADATA_NULLABLE).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "last_access",
                Datatype::Timestamp,
            ).set_meta(METADATA_ON_UPDATE, "current_timestamp()").set_default(Some(DefaultValue::Value(serde_json::Value::from("current_timestamp()")))).to_owned())
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "created",
                Datatype::Timestamp,
            ).set_default(Some(DefaultValue::Value(serde_json::Value::from("current_timestamp()")))).to_owned());



        assert_eq!(db.name(), db_name);
        assert_eq!(db.meta(METADATA_CHARSET), Some(String::from("utf8mb4")));
        assert_eq!(
            db.meta(METADATA_COLLATION),
            Some(String::from("utf8mb4_unicode_ci"))
        );
        assert_eq!(db.meta("shit"), None);

        let table_name = "test";
        let mut table = Table::new(table_name);
        table
            .set_column(Column::new(
                db_name,
                table_name,
                "test_id",
                Datatype::Int(10),
            ))
            .set_column(Column::new(
                db_name,
                table_name,
                "first_name",
                Datatype::Varchar(255),
            ))
            .set_column(Column::new(
                db_name,
                table_name,
                "last_name",
                Datatype::Varchar(255),
            ))
            .set_column(Column::new(
                db_name,
                table_name,
                "is_enabled",
                Datatype::Tinyint(1),
            ));

        table.set_index(Index::new("index_1", "test_id", false));

        let fk = Constraint::new("fk_1", "test_id", ("children", "test_id"));

        table.set_constraint(fk);

        db.set_table(table);

        assert!(db.table("test").is_some());
        assert_eq!(db.table("test").unwrap().name, "test");
        assert_eq!(db.table("test").unwrap().index("index_1").unwrap().name(), "index_1");
        assert_eq!(db.table("test").unwrap().constraint("fk_1").unwrap().local(), "test_id");
        assert_eq!(db.table("test").unwrap().constraint("fk_1").unwrap().foreign().1, "test_id");

        println!("{}", serde_json::to_string(&db).unwrap());
    }
}
