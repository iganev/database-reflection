use database_reflection::metadata::consts::*;
use database_reflection::metadata::WithMetadata;
use database_reflection::reflection::Column;
use database_reflection::reflection::Constraint;
use database_reflection::reflection::Database;
use database_reflection::reflection::Index;
use database_reflection::reflection::Table;
use database_reflection::reflection::{Datatype, DefaultValue};

fn get_mock_db () -> Database {
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
            Column::new(clients_table_name, "client_id", Datatype::Int(10))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_meta_flag(METADATA_FLAG_AUTO_INCREMENT)
                .to_owned(),
        )
        .set_column(Column::new(
            clients_table_name,
            "email",
            Datatype::Varchar(255),
        ))
        .set_column(Column::new(
            clients_table_name,
            "password",
            Datatype::Varchar(64),
        ))
        .set_column(
            Column::new(clients_table_name, "phone", Datatype::Varchar(45))
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(clients_table_name, "first_name", Datatype::Varchar(45))
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(clients_table_name, "last_name", Datatype::Varchar(45))
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(
                clients_table_name,
                "is_email_verified",
                Datatype::Tinyint(1),
            )
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(0))))
                .to_owned(),
        )
        .set_column(
            Column::new(
                clients_table_name,
                "email_verification_code",
                Datatype::Varchar(64),
            )
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(
                clients_table_name,
                "password_reset_code",
                Datatype::Varchar(64),
            )
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(clients_table_name, "last_access", Datatype::Timestamp)
                .set_meta(METADATA_ON_UPDATE, "current_timestamp()")
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
        )
        .set_column(
            Column::new(clients_table_name, "created", Datatype::Timestamp)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
        );

    if let Some(client_id_col) = clients_table.column("client_id") {
        clients_table.set_index(Index::new("PRIMARY", client_id_col.clone(), true, true));
    }

    if let Some(email_col) = clients_table.column("email") {
        clients_table.set_index(Index::new("email_UNIQUE", email_col.clone(), false, true));
    }

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
                client_tokens_table_name,
                "client_token_id",
                Datatype::Int(10),
            )
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_meta_flag(METADATA_FLAG_AUTO_INCREMENT)
                .to_owned(),
        )
        .set_column(
            Column::new(client_tokens_table_name, "client_id", Datatype::Int(10))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .to_owned(),
        )
        .set_column(Column::new(
            client_tokens_table_name,
            "auth_token",
            Datatype::Varchar(64),
        ))
        .set_column(Column::new(
            client_tokens_table_name,
            "auth_token_expiration_date",
            Datatype::Timestamp,
        ))
        .set_column(
            Column::new(
                client_tokens_table_name,
                "remote_address",
                Datatype::Varchar(64),
            )
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(
                client_tokens_table_name,
                "user_agent",
                Datatype::Varchar(255),
            )
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(client_tokens_table_name, "last_access", Datatype::Timestamp)
                .set_meta(METADATA_ON_UPDATE, "current_timestamp()")
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
        )
        .set_column(
            Column::new(client_tokens_table_name, "created", Datatype::Timestamp)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(
                    "current_timestamp()",
                ))))
                .to_owned(),
        );

    if let Some(client_token_id_col) = client_tokens_table.column("client_token_id") {
        client_tokens_table.set_index(Index::new(
            "PRIMARY",
            client_token_id_col.clone(),
            true,
            true,
        ));
    }

    if let Some(client_id_col) = client_tokens_table.column("client_id") {
        client_tokens_table.set_index(Index::new(
            "fk_client_tokens_1_idx",
            client_id_col.clone(),
            false,
            false,
        ));
    }

    if let Some(clients_table) = db.table("clients") {
        if let Some(client_id_col) = clients_table.column("client_id") {
            client_tokens_table.set_constraint(
                Constraint::new(
                    "fk_client_tokens_1",
                    client_tokens_table.column("client_id").unwrap(),
                    client_id_col,
                )
                    .set_meta(METADATA_ON_DELETE, METADATA_CASCADE)
                    .set_meta(METADATA_ON_UPDATE, METADATA_CASCADE)
                    .to_owned(),
            );
        }
    }

    client_tokens_table
        .set_meta(METADATA_CHARSET, "utf8mb4")
        .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

    db.set_table(client_tokens_table);

    //

    let products_table_name = "products";
    let mut products_table = Table::new(products_table_name);
    products_table
        .set_column(
            Column::new(products_table_name, "product_id", Datatype::Int(10))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_meta_flag(METADATA_FLAG_AUTO_INCREMENT)
                .to_owned(),
        )
        .set_column(
            Column::new(products_table_name, "name", Datatype::Varchar(255))
                .set_meta_flag(METADATA_FLAG_NULLABLE)
                .to_owned(),
        )
        .set_column(
            Column::new(products_table_name, "is_enabled", Datatype::Tinyint(1))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_default(Some(DefaultValue::Value(serde_json::Value::from(1))))
                .to_owned(),
        );

    if let Some(product_id_col) = products_table.column("product_id") {
        products_table.set_index(Index::new("PRIMARY", product_id_col.clone(), true, true));
    }

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
                client_products_table_name,
                "client_product_id",
                Datatype::Int(10),
            )
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .set_meta_flag(METADATA_FLAG_AUTO_INCREMENT)
                .to_owned(),
        )
        .set_column(
            Column::new(client_products_table_name, "client_id", Datatype::Int(10))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .to_owned(),
        )
        .set_column(
            Column::new(client_products_table_name, "product_id", Datatype::Int(10))
                .set_meta_flag(METADATA_FLAG_UNSIGNED)
                .to_owned(),
        );

    client_products_table.set_index(Index::new(
        "PRIMARY",
        client_products_table
            .column("client_product_id")
            .unwrap()
            .clone(),
        true,
        true,
    ));
    client_products_table.set_index(Index::new(
        "fk_client_products_1_idx",
        client_products_table.column("client_id").unwrap().clone(),
        false,
        false,
    ));
    client_products_table.set_index(Index::new(
        "fk_client_products_2_idx",
        client_products_table.column("product_id").unwrap().clone(),
        false,
        false,
    ));

    if let Some(clients_table) = db.table("clients") {
        if let Some(client_id_col) = clients_table.column("client_id") {
            client_products_table.set_constraint(
                Constraint::new(
                    "fk_client_products_1",
                    client_products_table.column("client_id").unwrap(),
                    client_id_col,
                )
                    .set_meta(METADATA_ON_DELETE, METADATA_CASCADE)
                    .set_meta(METADATA_ON_UPDATE, METADATA_CASCADE)
                    .to_owned(),
            );
        }
    }

    if let Some(products_table) = db.table("products") {
        if let Some(product_id_col) = products_table.column("product_id") {
            client_products_table.set_constraint(
                Constraint::new(
                    "fk_client_products_2",
                    client_products_table.column("product_id").unwrap(),
                    product_id_col,
                )
                    .set_meta(METADATA_ON_DELETE, METADATA_CASCADE)
                    .set_meta(METADATA_ON_UPDATE, METADATA_CASCADE)
                    .to_owned(),
            );
        }
    }

    client_products_table
        .set_meta(METADATA_CHARSET, "utf8mb4")
        .set_meta(METADATA_COLLATION, "utf8mb4_unicode_ci");

    db.set_table(client_products_table);

    db
}

#[test]
fn construction() {
    let db = get_mock_db();

    //

    for c in db
        .constraints_by_table(db.table("clients").unwrap(), None)
        .iter()
    {
        println!(
            "Constraint {}.{} -> {}.{}",
            c.local().table(),
            c.local().name(),
            c.foreign().table(),
            c.foreign().name()
        );
    }

    println!();

    for c in db
        .constraints_by_table(db.table("products").unwrap(), None)
        .iter()
    {
        println!(
            "Constraint {}.{} -> {}.{}",
            c.local().table(),
            c.local().name(),
            c.foreign().table(),
            c.foreign().name()
        );
    }

    println!();

    for c in db
        .constraints_by_column(
            db.table("client_products")
                .unwrap()
                .column("client_id")
                .unwrap(),
            None,
        )
        .iter()
    {
        println!(
            "Constraint {}.{} -> {}.{}",
            c.local().table(),
            c.local().name(),
            c.foreign().table(),
            c.foreign().name()
        );
    }

    println!();

    for c in db
        .constraints_by_column(
            db.table("products").unwrap().column("product_id").unwrap(),
            None,
        )
        .iter()
    {
        println!(
            "Constraint {}.{} -> {}.{}",
            c.local().table(),
            c.local().name(),
            c.foreign().table(),
            c.foreign().name()
        );
    }

    assert_eq!(db.name(), "test");
    assert_eq!(db.meta(METADATA_CHARSET), Some(String::from("utf8mb4")));
    assert_eq!(
        db.meta(METADATA_COLLATION),
        Some(String::from("utf8mb4_unicode_ci"))
    );
    assert_eq!(db.meta("shit"), None);

    println!("{}", serde_json::to_string(&db).unwrap());
}

#[test]
fn datatypes_test() {
    assert_eq!(Datatype::try_from("tinyint(1)"), Ok(Datatype::Tinyint(1)));
    assert_eq!(Datatype::try_from("int(10)"), Ok(Datatype::Int(10)));
    assert_eq!(Datatype::try_from("bigint(32)"), Ok(Datatype::Bigint(32)));
    assert_eq!(Datatype::try_from("float(4,2)"), Ok(Datatype::Float(4, 2)));
    assert_eq!(Datatype::try_from("real(10,2)"), Ok(Datatype::Real(10, 2)));

    assert_eq!(Datatype::try_from("date"), Ok(Datatype::Date));
    assert_eq!(Datatype::try_from("time"), Ok(Datatype::Time));
    assert_eq!(Datatype::try_from("datetime"), Ok(Datatype::Datetime));
    assert_eq!(Datatype::try_from("timestamp"), Ok(Datatype::Timestamp));

    assert_eq!(Datatype::try_from("char(64)"), Ok(Datatype::Char(64)));
    assert_eq!(Datatype::try_from("varchar(45)"), Ok(Datatype::Varchar(45)));
    assert_eq!(Datatype::try_from("text(1024)"), Ok(Datatype::Text(1024)));
    assert_eq!(Datatype::try_from("text"), Ok(Datatype::Text(65535)));

    assert_eq!(Datatype::try_from("binary(32)"), Ok(Datatype::Binary(32)));
    assert_eq!(
        Datatype::try_from("varbinary(32)"),
        Ok(Datatype::Varbinary(32))
    );

    assert_eq!(
        Datatype::try_from(r#"enum("one","two","three")"#),
        Ok(Datatype::Enum(vec![
            String::from("one"),
            String::from("two"),
            String::from("three")
        ]))
    );
    assert_eq!(
        Datatype::try_from(r#"set("this","that")"#),
        Ok(Datatype::Set(vec![
            String::from("this"),
            String::from("that")
        ]))
    );
}