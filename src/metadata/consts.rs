#[allow(dead_code)]
/// database, table or column charset
pub const METADATA_CHARSET: &str = "charset";
#[allow(dead_code)]
/// database, table or column collation
pub const METADATA_COLLATION: &str = "collation";
#[allow(dead_code)]
/// constraint action ON UPDATE
pub const METADATA_ON_UPDATE: &str = "on_update";
#[allow(dead_code)]
/// constraint action ON DELETE
pub const METADATA_ON_DELETE: &str = "on_delete";
#[allow(dead_code)]
/// constraint action value CASCADE
pub const METADATA_CASCADE: &str = "cascade";
#[allow(dead_code)]
/// constraint action value SET NULL
pub const METADATA_SET_NULL: &str = "set_null";

#[allow(dead_code)]
/// for numerical datatypes of columns
pub const METADATA_FLAG_UNSIGNED: &str = "unsigned";
#[allow(dead_code)]
/// for nullable columns
pub const METADATA_FLAG_NULLABLE: &str = "nullable";
#[allow(dead_code)]
/// for marking primary keys
pub const METADATA_FLAG_PRIMARY: &str = "primary";
#[allow(dead_code)]
/// primary key columns are usually AUTO INCREMENT
pub const METADATA_FLAG_AUTO_INCREMENT: &str = "auto_increment";
