[![Crates.io](https://img.shields.io/crates/v/database-reflection?color=4d76ae)](https://crates.io/crates/database-reflection)
[![API](https://docs.rs/database-reflection/badge.svg)](https://docs.rs/database-reflection)
[![dependency status](https://deps.rs/repo/github/iganev/database-reflection/status.svg)](https://deps.rs/repo/github/iganev/database-reflection)
[![build and test](https://github.com/iganev/database-reflection/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/database-reflection/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/iganev/database-reflection/graph/badge.svg?token=B5P2TAV5BB)](https://codecov.io/github/iganev/database-reflection)

# database-reflection
Collection of Rust structs and traits to construct a high-level database reflection.  
Currently under development. Subject to breaking changes and documentation struggles.  

For the time being the library supports only MariaDB with InnoDB table engine.  
Could theoretically work with MySQL and InnoDB but that is not confirmed as of time of writing.

Use at your own risk. Still under development.

## Quick Start

### Manual Construction
Take a look at test `integration.rs` to see how to build "by hand".

```rust
    use database_reflection::reflection::Database;

    let db_name = "test"; 
    let mut db = Database::new(db_name);

    // and so on...
```

### Using Reflection Adapter

If your database is designed by the convention this library expects, you may hope to be able to build a reflection using the provided reflection adapter for MariaDB.  
Note: this requires the `mariadb` feature to be enabled.

```rust
    use database_reflection::adapter::mariadb_innodb::MariadbInnodbReflectionAdapter;
    use database_reflection::adapter::reflection_adapter::{ReflectionAdapter, ReflectionAdapterUninitialized};
    use database_reflection::reflection::{Column, Constraint, Database, DefaultValue, Index, SqlDatatype, Table};

    dotenvy::dotenv()?;
    let connection_str = env::var("DB_CONNECTION")?;
    let reflection = MariadbInnodbReflectionAdapter::new(&connection_str).connect().await?.get_reflection().await?;
```

Note: `connection_str` is a standard DSN.

## License

This library (database-reflection) is open sourced under the MIT License. 
