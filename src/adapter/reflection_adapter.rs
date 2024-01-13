use crate::reflection::{Database, Table};
use sqlx::{Error, Pool};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Clone, Default, Debug)]
pub struct Uninitialized;
#[derive(Clone, Debug)]
pub struct Connected<DB: sqlx::Database>(Pool<DB>);

pub trait State {}

impl State for Uninitialized {}

impl<DB: sqlx::Database> State for Connected<DB> {}
impl<DB: sqlx::Database> Deref for Connected<DB> {
    type Target = Pool<DB>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<DB: sqlx::Database> Connected<DB> {
    pub fn new(pool: Pool<DB>) -> Connected<DB> {
        Connected(pool)
    }
}

#[derive(Debug)]
pub enum ReflectionAdapterError {
    ConnectionError(Error),
    DatabaseError(Error),
    ValidationError(String),
    IntegrityError(String),
}

impl Display for ReflectionAdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectionAdapterError::ConnectionError(e) => {
                write!(f, "ConnectionError: {}", e.to_string())
            }
            ReflectionAdapterError::DatabaseError(e) => {
                write!(f, "DatabaseError: {}", e.to_string())
            }
            ReflectionAdapterError::ValidationError(e) => write!(f, "ConnectionError: {}", e),
            ReflectionAdapterError::IntegrityError(e) => write!(f, "ConnectionError: {}", e),
        }
    }
}

impl std::error::Error for ReflectionAdapterError {}

pub trait ReflectionAdapterUninitialized<T> {
    fn set_connection_string(&mut self, connection_string: &str);

    fn connect(self)
        -> impl std::future::Future<Output = Result<T, ReflectionAdapterError>> + Send;
}

pub trait ReflectionAdapter<T> {
    fn disconnect(
        self,
    ) -> impl std::future::Future<Output = Result<T, ReflectionAdapterError>> + Send;

    fn set_database_name(
        &mut self,
        database_name: &str,
    ) -> impl std::future::Future<Output = Result<(), ReflectionAdapterError>>;

    fn get_database_name(&self) -> &str;

    fn list_database_names(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, ReflectionAdapterError>> + Send;

    fn list_table_names(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, ReflectionAdapterError>> + Send;

    fn get_table_reflection(
        &self,
        table_name: &str,
    ) -> impl std::future::Future<Output = Result<Table, ReflectionAdapterError>> + Send;

    fn get_reflection(
        &self,
    ) -> impl std::future::Future<Output = Result<Database, ReflectionAdapterError>> + Send;
}
