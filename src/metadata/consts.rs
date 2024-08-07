#![allow(dead_code)]
/// database, table or column charset
pub const METADATA_CHARSET: &str = "charset";

/// database, table or column collation
pub const METADATA_COLLATION: &str = "collation";

/// constraint action ON UPDATE or TIMESTAMP internal update
pub const METADATA_ON_UPDATE: &str = "on_update";

/// constraint action ON DELETE
pub const METADATA_ON_DELETE: &str = "on_delete";

/// constraint action value CASCADE
pub const METADATA_CASCADE: &str = "cascade";

/// constraint action value SET NULL
pub const METADATA_SET_NULL: &str = "set_null";


/// for numerical datatypes of columns
pub const METADATA_FLAG_UNSIGNED: &str = "unsigned";

/// for nullable columns
pub const METADATA_FLAG_NULLABLE: &str = "nullable";

/// for marking primary keys
pub const METADATA_FLAG_PRIMARY: &str = "primary";
/// for marking unique indexes
pub const METADATA_FLAG_UNIQUE: &str = "unique";

/// primary key columns are usually AUTO INCREMENT
pub const METADATA_FLAG_AUTO_INCREMENT: &str = "auto_increment";

/// TIMESTAMP DEFAULT
pub const METADATA_FLAG_DEFAULT_CURRENT_TIMESTAMP: &str = "current_timestamp()";

/// TIMESTAMP ON UPDATE trigger
pub const METADATA_FLAG_ON_UPDATE_CURRENT_TIMESTAMP: &str = "on update current_timestamp()";
