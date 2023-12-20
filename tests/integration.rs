use database_reflection::metadata::consts::*;
use database_reflection::metadata::WithMetadata;
use database_reflection::reflection::Constraint;
use database_reflection::reflection::Database;
use database_reflection::reflection::Index;
use database_reflection::reflection::Table;
use database_reflection::reflection::{Column, ConstraintSide};
use database_reflection::reflection::{Datatype, DefaultValue};

fn get_mock_db() -> Database {
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
                .set_meta_flag(METADATA_FLAG_PRIMARY)
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
            .set_meta_flag(METADATA_FLAG_PRIMARY)
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
                .set_meta_flag(METADATA_FLAG_PRIMARY)
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
            //.set_meta_flag(METADATA_FLAG_PRIMARY)
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

    assert!(db
        .table("clients")
        .unwrap()
        .column("phone")
        .unwrap()
        .meta_flag(METADATA_FLAG_NULLABLE));

    assert_eq!(
        db.table("clients").unwrap().meta(METADATA_CHARSET),
        Some("utf8mb4".to_string())
    );

    let col_list = vec!["client_product_id", "client_id", "product_id"];
    for (column_name, column) in db.table("client_products").unwrap().columns() {
        assert_eq!(column_name.as_str(), column.name().as_str());
        assert!(col_list.contains(&column_name.as_str()));
    }

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .constraint("fk_client_products_1")
            .unwrap()
            .foreign()
            .table()
            .as_str(),
        "clients"
    );

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .constraint_by_column_name(String::from("client_id").into())
            .unwrap()
            .foreign()
            .table()
            .as_str(),
        "clients"
    );

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .constraint_by_column(
                db.table("client_products")
                    .unwrap()
                    .column("client_id")
                    .unwrap()
                    .as_ref()
            )
            .unwrap()
            .foreign()
            .table()
            .as_str(),
        "clients"
    );

    //

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .index("PRIMARY")
            .unwrap()
            .column()
            .name()
            .as_str(),
        "client_product_id"
    );

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .index_by_column_name(String::from("client_id").into())
            .unwrap()
            .column()
            .name()
            .as_str(),
        "client_id"
    );

    assert_eq!(
        db.table("client_products")
            .unwrap()
            .index_by_column(
                db.table("client_products")
                    .unwrap()
                    .column("client_id")
                    .unwrap()
                    .as_ref()
            )
            .unwrap()
            .column()
            .name()
            .as_str(),
        "client_id"
    );

    //

    assert_eq!(db.table("client_products").unwrap().constraints().len(), 2);

    let constr_list = vec!["fk_client_products_1", "fk_client_products_2"];
    for (constr_name, constr) in db.table("client_products").unwrap().constraints() {
        assert_eq!(constr_name.as_str(), constr.name().as_str());
        assert!(constr_list.contains(&constr_name.as_str()));
    }

    //

    assert_eq!(db.table("client_products").unwrap().indexes().len(), 3);

    let idx_list = vec![
        "PRIMARY",
        "fk_client_products_1_idx",
        "fk_client_products_2_idx",
    ];
    for (idx_name, idx) in db.table("client_products").unwrap().indexes() {
        assert_eq!(idx_name.as_str(), idx.name().as_str());
        assert!(idx_list.contains(&idx_name.as_str()));
    }

    for (_, table) in db.tables() {
        assert_eq!(table.primary_key_count(), 1);
    }

    assert_eq!(
        db.table("clients").unwrap().primary_key(),
        Some("client_id".to_string().into())
    );
    assert_eq!(
        db.table("clients").unwrap().primary_key_column(),
        db.table("clients").unwrap().column("client_id")
    );

    assert_eq!(db.table("clients").unwrap().primary_keys().len(), 1);
    assert_eq!(
        db.table("clients").unwrap().primary_keys().next().cloned(),
        Some("client_id".to_string().into())
    );
    assert_eq!(
        db.table("clients")
            .unwrap()
            .primary_key_columns()
            .first()
            .cloned(),
        db.table("clients").unwrap().column("client_id")
    );

    //

    assert_eq!(db.constraints().len(), 3);

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
        .constraints_by_table(
            db.table("client_products").unwrap(),
            Some(ConstraintSide::Local),
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
