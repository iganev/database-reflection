use indexmap::IndexMap;
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
#[allow(dead_code)]
const METADATA_AUTO_INCREMENT: &str = "auto_increment";

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

pub trait WithMetadata {
    fn get_metadata(&self) -> &HashMap<String, String>;

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String>;

    fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Self {
        self.get_metadata_mut()
            .insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    fn set_meta_flag(&mut self, meta_flag: impl ToString) -> &mut Self {
        self.get_metadata_mut()
            .insert(meta_flag.to_string(), "1".to_string());

        self
    }

    fn meta_flag(&self, flag: &str) -> bool {
        self.get_metadata().contains_key(flag)
    }

    fn meta(&self, key: &str) -> Option<String> {
        self.get_metadata().get(key).cloned()
    }
}

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database<'n> {
    name: &'n str,
    tables: IndexMap<String, Table<'n>>,
    constraints: HashMap<String, Constraint<'n>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Database<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
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
    columns: IndexMap<String, Column<'n>>,
    constraints: HashMap<String, Constraint<'n>>,
    indexes: IndexMap<String, Index<'n>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Table<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
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

impl<'n> WithMetadata for Column<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
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
    column: &'n str,
    primary: bool,
    unique: bool,
}

impl<'n> Index<'n> {
    pub fn new(name: &'n str, column: &'n str, primary: bool, unique: bool) -> Self {
        Index {
            name,
            column,
            primary,
            unique,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn column(&self) -> &str {
        self.column
    }

    pub fn primary(&self) -> bool {
        self.primary
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

impl<'n> WithMetadata for Constraint<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
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

        //

        let clients_table_name = "clients";
        let mut clients_table = Table::new(clients_table_name);
        clients_table
            .set_column(
                Column::new(db_name, clients_table_name, "client_id", Datatype::Int(10))
                    .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                    .set_meta(METADATA_AUTO_INCREMENT, METADATA_AUTO_INCREMENT)
                    .to_owned(),
            )
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
            .set_column(
                Column::new(db_name, clients_table_name, "phone", Datatype::Varchar(45))
                    .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                    .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "first_name",
                    Datatype::Varchar(45),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "last_name",
                    Datatype::Varchar(45),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "is_email_verified",
                    Datatype::Tinyint(1),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(0))))
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "email_verification_code",
                    Datatype::Varchar(64),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "password_reset_code",
                    Datatype::Varchar(64),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "last_access",
                    Datatype::Timestamp,
                )
                .set_meta(METADATA_ON_UPDATE, "current_timestamp()")
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
            )
            .set_column(
                Column::new(db_name, clients_table_name, "created", Datatype::Timestamp)
                    .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                        "current_timestamp()",
                    ))))
                    .to_owned(),
            );

        clients_table.set_index(Index::new("PRIMARY", "client_id", true, true));
        clients_table.set_index(Index::new("email_UNIQUE", "email", false, true));
        clients_table
            .set_meta(METADATA_CHARSET, "utf8mb4")
            .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        db.set_table(clients_table);

        //

        let client_tokens_table_name = "client_tokens";
        let mut client_tokens_table = Table::new(client_tokens_table_name);
        client_tokens_table
            .set_column(
                Column::new(
                    db_name,
                    client_tokens_table_name,
                    "client_token_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .set_meta(METADATA_AUTO_INCREMENT, METADATA_AUTO_INCREMENT)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    client_tokens_table_name,
                    "client_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .to_owned(),
            )
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "auth_token",
                Datatype::Varchar(64),
            ))
            .set_column(Column::new(
                db_name,
                clients_table_name,
                "auth_token_expiration_date",
                Datatype::Timestamp,
            ))
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "remote_address",
                    Datatype::Varchar(64),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "user_agent",
                    Datatype::Varchar(255),
                )
                .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    clients_table_name,
                    "last_access",
                    Datatype::Timestamp,
                )
                .set_meta(METADATA_ON_UPDATE, "current_timestamp()")
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
            )
            .set_column(
                Column::new(db_name, clients_table_name, "created", Datatype::Timestamp)
                    .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                        "current_timestamp()",
                    ))))
                    .to_owned(),
            );

        client_tokens_table.set_index(Index::new("PRIMARY", "client_token_id", true, true));
        client_tokens_table.set_index(Index::new(
            "fk_client_tokens_1_idx",
            "client_id",
            false,
            false,
        ));
        client_tokens_table.set_constraint(Constraint::new(
            "fk_client_tokens_1",
            "client_id",
            ("clients", "client_id"),
        ));
        client_tokens_table
            .set_meta(METADATA_CHARSET, "utf8mb4")
            .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        db.set_table(client_tokens_table);

        //

        let products_table_name = "products";
        let mut products_table = Table::new(clients_table_name);
        products_table
            .set_column(
                Column::new(
                    db_name,
                    products_table_name,
                    "product_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .set_meta(METADATA_AUTO_INCREMENT, METADATA_AUTO_INCREMENT)
                .to_owned(),
            )
            .set_column(
                Column::new(db_name, products_table_name, "name", Datatype::Varchar(255))
                    .set_meta(METADATA_NULLABLE, METADATA_NULLABLE)
                    .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    products_table_name,
                    "is_enabled",
                    Datatype::Tinyint(1),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(1))))
                .to_owned(),
            );

        products_table.set_index(Index::new("PRIMARY", "product_id", true, true));
        products_table
            .set_meta(METADATA_CHARSET, "utf8mb4")
            .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        db.set_table(products_table);

        //

        let client_products_table_name = "client_products";
        let mut client_products_table = Table::new(client_products_table_name);
        client_products_table
            .set_column(
                Column::new(
                    db_name,
                    client_products_table_name,
                    "client_product_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .set_meta(METADATA_AUTO_INCREMENT, METADATA_AUTO_INCREMENT)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    client_products_table_name,
                    "client_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .to_owned(),
            )
            .set_column(
                Column::new(
                    db_name,
                    client_products_table_name,
                    "product_id",
                    Datatype::Int(10),
                )
                .set_meta(METADATA_UNSIGNED, METADATA_UNSIGNED)
                .to_owned(),
            );

        client_products_table.set_index(Index::new("PRIMARY", "client_product_id", true, true));
        client_products_table.set_index(Index::new(
            "fk_client_products_1_idx",
            "client_id",
            false,
            false,
        ));
        client_products_table.set_index(Index::new(
            "fk_client_products_2_idx",
            "product_id",
            false,
            false,
        ));
        client_products_table.set_constraint(Constraint::new(
            "fk_client_products_1",
            "client_id",
            ("clients", "client_id"),
        ));
        client_products_table.set_constraint(Constraint::new(
            "fk_client_products_2",
            "product_id",
            ("products", "product_id"),
        ));
        client_products_table
            .set_meta(METADATA_CHARSET, "utf8mb4")
            .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

        db.set_table(client_products_table);

        //

        assert_eq!(db.name(), db_name);
        assert_eq!(db.meta(METADATA_CHARSET), Some(String::from("utf8mb4")));
        assert_eq!(
            db.meta(METADATA_COLLATION),
            Some(String::from("utf8mb4_unicode_ci"))
        );
        assert_eq!(db.meta("shit"), None);

        println!("{}", serde_json::to_string(&db).unwrap());
    }
}
