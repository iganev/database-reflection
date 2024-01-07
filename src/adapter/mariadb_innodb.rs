use crate::adapter::reflection_adapter::ReflectionAdapterError::DatabaseError;
use crate::adapter::reflection_adapter::{
    Connected, ReflectionAdapter, ReflectionAdapterError, ReflectionAdapterUninitialized, State,
    Uninitialized,
};
use crate::metadata::consts::{
    METADATA_CHARSET, METADATA_COLLATION, METADATA_FLAG_AUTO_INCREMENT,
    METADATA_FLAG_DEFAULT_CURRENT_TIMESTAMP, METADATA_FLAG_NULLABLE,
    METADATA_FLAG_ON_UPDATE_CURRENT_TIMESTAMP, METADATA_FLAG_PRIMARY, METADATA_FLAG_UNIQUE,
};
use crate::metadata::WithMetadata;
use crate::reflection::{Column, Constraint, Database, DefaultValue, Index, SqlDatatype, Table};
use serde_json::Value;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

#[derive(Clone, Default, Debug)]
pub struct MariadbInnodbReflectionAdapter<T: State> {
    state: T,
    connection_string: String,
    database_name: String,
}

impl MariadbInnodbReflectionAdapter<Uninitialized> {
    pub fn new(connection_string: &str) -> MariadbInnodbReflectionAdapter<Uninitialized> {
        MariadbInnodbReflectionAdapter::<Uninitialized> {
            connection_string: connection_string.to_string(),
            ..Default::default()
        }
    }
}

impl ReflectionAdapterUninitialized<MariadbInnodbReflectionAdapter<Connected<MySql>>>
    for MariadbInnodbReflectionAdapter<Uninitialized>
{
    fn set_connection_string(&mut self, connection_string: &str) {
        self.connection_string = connection_string.to_string();
    }

    async fn connect(
        self,
    ) -> Result<MariadbInnodbReflectionAdapter<Connected<MySql>>, ReflectionAdapterError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&self.connection_string)
            .await
            .map_err(|e| ReflectionAdapterError::ConnectionError(e))?;

        let database_name: String = sqlx::query_scalar("SELECT DATABASE()")
            .fetch_one(&pool)
            .await
            .unwrap_or_default();

        Ok(MariadbInnodbReflectionAdapter::<Connected<MySql>> {
            state: Connected::new(pool),
            connection_string: self.connection_string,
            database_name,
        })
    }
}

impl MariadbInnodbReflectionAdapter<Connected<MySql>> {
    pub fn get_connection(&self) -> &Pool<MySql> {
        &*self.state
    }
}

impl ReflectionAdapter<MariadbInnodbReflectionAdapter<Uninitialized>>
    for MariadbInnodbReflectionAdapter<Connected<MySql>>
{
    async fn disconnect(
        self,
    ) -> Result<MariadbInnodbReflectionAdapter<Uninitialized>, ReflectionAdapterError> {
        self.get_connection().close().await;

        Ok(MariadbInnodbReflectionAdapter::new(&self.connection_string))
    }

    async fn set_database_name(
        &mut self,
        database_name: &str,
    ) -> Result<(), ReflectionAdapterError> {
        sqlx::query(format!("USE  {}", &database_name).as_str())
            .execute(&*self.state)
            .await
            .map_err(|e| DatabaseError(e))?;

        self.database_name = database_name.to_string();

        Ok(())
    }

    fn get_database_name(&self) -> &str {
        &self.database_name
    }

    async fn list_table_names(&self) -> Result<Vec<String>, ReflectionAdapterError> {
        sqlx::query_scalar("SHOW TABLES")
            .fetch_all(self.get_connection())
            .await
            .map_err(|e| DatabaseError(e))
    }

    async fn get_table_reflection(
        &self,
        table_name: &str,
    ) -> Result<Table, ReflectionAdapterError> {
        let mut table = Table::new(table_name);

        let default_table_character_set_and_collation:(Option<String>, Option<String>) = sqlx::query_as(format!("SELECT CCSA.CHARACTER_SET_NAME, CCSA.COLLATION_NAME FROM information_schema.`TABLES` T, information_schema.`COLLATION_CHARACTER_SET_APPLICABILITY` CCSA WHERE CCSA.COLLATION_NAME = T.TABLE_COLLATION AND T.TABLE_SCHEMA = '{}' AND T.TABLE_NAME = '{}'", &self.database_name, table_name).as_str()).fetch_one(self.get_connection()).await.map_err(|e| DatabaseError(e))?;
        if let (Some(charset), Some(collation)) = default_table_character_set_and_collation {
            table
                .set_meta(METADATA_CHARSET, charset)
                .set_meta(METADATA_COLLATION, collation);
        }

        let table_columns: Vec<(
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        )> = sqlx::query_as(format!("SHOW COLUMNS FROM {}", table_name).as_str())
            .fetch_all(self.get_connection())
            .await
            .map_err(|e| DatabaseError(e))?;
        for table_column in table_columns {
            let (field_name, field_type, field_nullable, field_key, field_default, field_extra) =
                table_column;

            let mut col = Column::new(
                table.name().as_ref(),
                field_name.as_str(),
                SqlDatatype::try_from(field_type.as_str()).unwrap_or(SqlDatatype::default()),
            );

            if let SqlDatatype::Char(_) | SqlDatatype::Varchar(_) | SqlDatatype::Text(_) =
                col.datatype()
            {
                let default_column_character_set_and_collation:(Option<String>, Option<String>) = sqlx::query_as(format!("SELECT CHARACTER_SET_NAME, COLLATION_NAME FROM information_schema.`COLUMNS` WHERE table_schema = '{}' AND table_name = '{}' AND column_name = '{}'", &self.database_name, table_name, field_name.as_str()).as_str()).fetch_one(self.get_connection()).await.map_err(|e| DatabaseError(e))?;

                if let (Some(charset), Some(collation)) = default_column_character_set_and_collation
                {
                    col.set_meta(METADATA_CHARSET, charset)
                        .set_meta(METADATA_COLLATION, collation);
                }
            }

            if field_nullable == "YES" {
                col.set_meta_flag(METADATA_FLAG_NULLABLE);
            }

            if let Some(key_designation) = field_key {
                match key_designation.as_str() {
                    "PRI" => {
                        col.set_meta_flag(METADATA_FLAG_PRIMARY);
                    }
                    "UNI" => {
                        col.set_meta_flag(METADATA_FLAG_UNIQUE);

                        //dig out index

                        //TODO
                    }
                    "MUL" => {
                        //dig out constraint

                        //TODO
                    }
                    _ => {}
                }
            }

            if let Some(default_value) = field_default {
                if col.datatype() == &SqlDatatype::Timestamp
                    && default_value == METADATA_FLAG_DEFAULT_CURRENT_TIMESTAMP
                {
                    col.set_meta_flag(METADATA_FLAG_DEFAULT_CURRENT_TIMESTAMP);
                }

                col.set_default(Some(DefaultValue::Value(Value::from(default_value))));
            }

            if let Some(extra) = field_extra {
                if extra.len() > 0 {
                    if extra.as_str() == METADATA_FLAG_AUTO_INCREMENT {
                        col.set_meta_flag(METADATA_FLAG_AUTO_INCREMENT);
                    } else if extra.as_str() == METADATA_FLAG_ON_UPDATE_CURRENT_TIMESTAMP {
                        col.set_meta_flag(METADATA_FLAG_ON_UPDATE_CURRENT_TIMESTAMP);
                    }
                }
            }

            table.set_column(col);
        }

        //Table 	Non_unique 	Key_name 	Seq_in_index 	Column_name 	Collation 	Cardinality 	Sub_part 	Packed 	Null 	Index_type 	Comment 	Index_comment 	Ignored
        let table_indexes: Vec<(
            String,
            bool,
            String,
            i32,
            String,
            String,
            i32,
            Option<String>,
            Option<String>,
            String,
            String,
            String,
            String,
            String,
        )> = sqlx::query_as(format!("SHOW INDEXES FROM {}", table_name).as_str())
            .fetch_all(self.get_connection())
            .await
            .map_err(|e| DatabaseError(e))?;

        for table_index in table_indexes {
            let (
                _index_table_name,
                index_unique,
                index_name,
                _index_sequence,
                index_column,
                _index_collation,
                _index_cardinality,
                _index_subpart,
                _index_packed,
                _index_null,
                _index_type,
                _index_comment,
                _index_comment2,
                _index_ignored,
            ) = table_index;

            if let Some(col) = table.column(&index_column) {
                let primary = index_name == "PRIMARY";
                let indx = Index::new(index_name, col, primary, index_unique);

                table.set_index(indx);
            }
        }

        Ok(table)
    }

    async fn get_reflection(&self) -> Result<Database, ReflectionAdapterError> {
        let mut db = Database::new(&self.database_name);

        // collect tables
        let tables = self.list_table_names().await?;
        for table_name in tables {
            let table = self.get_table_reflection(&table_name).await?;

            db.set_table(table);
        }

        //TODO InnoDB check
        // SELECT TABLE_NAME, ENGINE FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'exams';

        // collect constraints
        let foreign_keys: Vec<(String, String, String, Option<String>, Option<String>)> =
            sqlx::query_as(
                format!(
                    r#"
        SELECT
          CONSTRAINT_NAME, TABLE_NAME, COLUMN_NAME, REFERENCED_TABLE_NAME, REFERENCED_COLUMN_NAME
        FROM
          INFORMATION_SCHEMA.KEY_COLUMN_USAGE
        WHERE
          TABLE_SCHEMA = '{}' AND
          CONSTRAINT_NAME != 'PRIMARY' AND
          REFERENCED_TABLE_SCHEMA = TABLE_SCHEMA
        ORDER BY CONSTRAINT_NAME ASC, POSITION_IN_UNIQUE_CONSTRAINT ASC
        "#,
                    &self.database_name
                )
                .as_str(),
            )
            .fetch_all(self.get_connection())
            .await
            .map_err(|e| DatabaseError(e))?;

        for foreign_key in foreign_keys {
            let (
                fk_name,
                local_table_name,
                local_column_name,
                foreign_table_name,
                foreign_column_name,
            ) = foreign_key;

            if let Some(local_table) = db.table(&local_table_name) {
                if let Some(local_column) = local_table.column(&local_column_name) {
                    if let Some(foreign_table) =
                        db.table(foreign_table_name.unwrap_or_default().as_str())
                    {
                        if let Some(foreign_column) =
                            foreign_table.column(foreign_column_name.unwrap_or_default().as_str())
                        {
                            let constraint = if let Some(constraint) = db.constraint(&fk_name) {
                                let mut c = (*constraint).clone();
                                c.add_key_pair(local_column, foreign_column);

                                c
                            } else {
                                Constraint::new(fk_name, local_column, foreign_column)
                            };

                            db.set_constraint(constraint);
                        }
                    }
                }
            }
        }

        Ok(db)
    }
}
