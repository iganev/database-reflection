use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum SqlSigned {
    #[default]
    Signed,
    Unsigned,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Basic SQL datatypes, defaults to VARCHAR(45)
pub enum SqlDatatype {
    Tinyint(u32, SqlSigned),
    Smallint(u32, SqlSigned),
    Mediumint(u32, SqlSigned),
    Int(u32, SqlSigned),
    Bigint(u32, SqlSigned),
    Float(u32, u32, SqlSigned),
    Double(u32, u32, SqlSigned),
    Decimal(u32, u32, SqlSigned),

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
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDatatypeError;

impl SqlDatatype {
    /// Get datatype sign
    pub fn sign(&self) -> Option<SqlSigned> {
        match self {
            SqlDatatype::Tinyint(_, sign) => Some(sign.clone()),
            SqlDatatype::Smallint(_, sign) => Some(sign.clone()),
            SqlDatatype::Mediumint(_, sign) => Some(sign.clone()),
            SqlDatatype::Int(_, sign) => Some(sign.clone()),
            SqlDatatype::Bigint(_, sign) => Some(sign.clone()),
            SqlDatatype::Float(_, _, sign) => Some(sign.clone()),
            SqlDatatype::Double(_, _, sign) => Some(sign.clone()),
            SqlDatatype::Decimal(_, _, sign) => Some(sign.clone()),
            _ => None,
        }
    }

    /// Get datatype length
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> Option<u32> {
        match self {
            SqlDatatype::Tinyint(len, _) => Some(*len),
            SqlDatatype::Smallint(len, _) => Some(*len),
            SqlDatatype::Mediumint(len, _) => Some(*len),
            SqlDatatype::Int(len, _) => Some(*len),
            SqlDatatype::Bigint(len, _) => Some(*len),
            SqlDatatype::Float(len, _, _) => Some(*len),
            SqlDatatype::Double(len, _, _) => Some(*len),
            SqlDatatype::Decimal(len, _, _) => Some(*len),
            SqlDatatype::Char(len) => Some(*len),
            SqlDatatype::Varchar(len) => Some(*len),
            SqlDatatype::Text(len) => Some(*len),
            SqlDatatype::Binary(len) => Some(*len),
            SqlDatatype::Varbinary(len) => Some(*len),
            SqlDatatype::Enum(v) => Some(v.len() as u32),
            SqlDatatype::Set(v) => Some(v.len() as u32),
            _ => None,
        }
    }

    /// Check if column datatype is one of the character types
    pub fn is_text(&self) -> bool {
        matches!(self, SqlDatatype::Text(_) | SqlDatatype::Varchar(_) | SqlDatatype::Char(_))
    }
}

impl Default for SqlDatatype {
    fn default() -> Self {
        SqlDatatype::Varchar(45)
    }
}

impl FromStr for SqlDatatype {
    type Err = ParseDatatypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SqlDatatype::try_from(s)
    }
}

impl TryFrom<&str> for SqlDatatype {
    type Error = ParseDatatypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return if value.contains('(') && value.contains(')') {
            // type with length information

            let is_signed = if value.contains("unsigned") {
                SqlSigned::Unsigned
            } else {
                SqlSigned::Signed
            };

            let (type_group, type_length) = value
                .split_once('(')
                .map(|s| (s.0, s.1.rsplit_once(')').unwrap_or(("", "")).0))
                .ok_or(ParseDatatypeError)?;

            match type_group {
                "set" | "enum" => {
                    let trim_match: &[_] = &['"', '\''];
                    let options = type_length
                        .split(',')
                        .map(|s| s.trim_matches(trim_match).to_string())
                        .collect();

                    return if type_group == "set" {
                        Ok(SqlDatatype::Set(options))
                    } else {
                        Ok(SqlDatatype::Enum(options))
                    };
                }
                "float" | "double" | "decimal" => {
                    let (left, right) = {
                        let v = type_length
                            .splitn(2, ',')
                            .map(|s| s.parse::<u32>().unwrap_or_default())
                            .collect::<Vec<u32>>();

                        (
                            *v.first().ok_or(ParseDatatypeError)?,
                            *v.last().ok_or(ParseDatatypeError)?,
                        )
                    };

                    if type_group == "float" {
                        Ok(SqlDatatype::Float(left, right, is_signed))
                    } else if type_group == "double" {
                        Ok(SqlDatatype::Double(left, right, is_signed))
                    } else {
                        Ok(SqlDatatype::Decimal(left, right, is_signed))
                    }
                }
                _ => {
                    if let Ok(len_val) = type_length.parse::<u32>() {
                        match type_group {
                            "tinyint" => Ok(SqlDatatype::Tinyint(len_val, is_signed)),
                            "int" => Ok(SqlDatatype::Int(len_val, is_signed)),
                            "smallint" => Ok(SqlDatatype::Smallint(len_val, is_signed)),
                            "mediumint" => Ok(SqlDatatype::Mediumint(len_val, is_signed)),
                            "bigint" => Ok(SqlDatatype::Bigint(len_val, is_signed)),

                            "char" => Ok(SqlDatatype::Char(len_val)),
                            "varchar" => Ok(SqlDatatype::Varchar(len_val)),
                            "text" => Ok(SqlDatatype::Text(len_val)), // can be without length

                            "binary" => Ok(SqlDatatype::Binary(len_val)),
                            "varbinary" => Ok(SqlDatatype::Varbinary(len_val)),

                            _ => Err(ParseDatatypeError),
                        }
                    } else {
                        Err(ParseDatatypeError)
                    }
                }
            }
        } else {
            // fixed length type

            match value {
                "text" => Ok(SqlDatatype::Text(65535)),
                "date" => Ok(SqlDatatype::Date),
                "time" => Ok(SqlDatatype::Time),
                "datetime" => Ok(SqlDatatype::Datetime),
                "timestamp" => Ok(SqlDatatype::Timestamp),
                _ => Err(ParseDatatypeError),
            }
        };
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Default value container
pub enum DefaultValue {
    #[default]
    Null,
    Value(Value),
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// JS Number
pub enum JsonNumber {
    #[default]
    Number,
    BigInt,
    Int,
    Float,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// JS String
pub enum JsonString {
    #[default]
    String,
    Datetime,
    Date,
    Time,
    Json,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Basic JSON datatypes, defaults to string
pub enum JsonDatatype {
    Number(JsonNumber),
    String(JsonString, Option<u32>),
    Boolean,
    Array(Vec<String>),
    Object(Value),
}

impl Default for JsonDatatype {
    fn default() -> Self {
        JsonDatatype::String(JsonString::String, None)
    }
}

impl From<&SqlDatatype> for JsonDatatype {
    fn from(value: &SqlDatatype) -> Self {
        match value {
            SqlDatatype::Tinyint(1, SqlSigned::Unsigned) => JsonDatatype::Boolean,
            SqlDatatype::Int(_, _)
            | SqlDatatype::Smallint(_, _)
            | SqlDatatype::Mediumint(_, _)
            | SqlDatatype::Tinyint(_, _) => JsonDatatype::Number(JsonNumber::Int),
            SqlDatatype::Float(_, _, _)
            | SqlDatatype::Double(_, _, _)
            | SqlDatatype::Decimal(_, _, _) => JsonDatatype::Number(JsonNumber::Float),
            SqlDatatype::Bigint(_, _) => JsonDatatype::Number(JsonNumber::BigInt),
            SqlDatatype::Date => JsonDatatype::String(JsonString::Date, Some(10)),
            SqlDatatype::Time => JsonDatatype::String(JsonString::Time, Some(8)),
            SqlDatatype::Datetime | SqlDatatype::Timestamp => {
                JsonDatatype::String(JsonString::Datetime, Some(20))
            }
            SqlDatatype::Char(length)
            | SqlDatatype::Varchar(length)
            | SqlDatatype::Text(length) => JsonDatatype::String(JsonString::String, Some(*length)),
            SqlDatatype::Binary(length) | SqlDatatype::Varbinary(length) => {
                JsonDatatype::String(JsonString::String, Some(*length))
            }
            SqlDatatype::Enum(options) | SqlDatatype::Set(options) => {
                JsonDatatype::Array(options.clone())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Basic Rust type as string
pub struct RustDatatype(pub String, pub Option<u32>);

pub const RUST_TYPE_STRING: &str = "String";
//pub const RUST_TYPE_USIZE: &str = "usize";
//pub const RUST_TYPE_ISIZE: &str = "isize";
pub const RUST_TYPE_I32: &str = "i32";
pub const RUST_TYPE_I64: &str = "i64";
pub const RUST_TYPE_U32: &str = "u32";
pub const RUST_TYPE_U64: &str = "u64";
pub const RUST_TYPE_F32: &str = "f32";
pub const RUST_TYPE_F64: &str = "f64";
pub const RUST_TYPE_BOOL: &str = "bool";
pub const RUST_TYPE_VEC: &str = "Vec<String>";

impl Default for RustDatatype {
    fn default() -> Self {
        RustDatatype(RUST_TYPE_STRING.to_string(), None)
    }
}

impl From<&SqlDatatype> for RustDatatype {
    fn from(value: &SqlDatatype) -> Self {
        match value {
            SqlDatatype::Tinyint(1, _) => {
                // SqlSigned::Unsigned, pattern removed to patch database bugs by code...
                RustDatatype(RUST_TYPE_BOOL.to_string(), None)
            }
            SqlDatatype::Int(len, SqlSigned::Unsigned)
            | SqlDatatype::Smallint(len, SqlSigned::Unsigned)
            | SqlDatatype::Mediumint(len, SqlSigned::Unsigned)
            | SqlDatatype::Tinyint(len, SqlSigned::Unsigned) => {
                RustDatatype(RUST_TYPE_U32.to_string(), Some(*len))
            }
            SqlDatatype::Int(len, SqlSigned::Signed)
            | SqlDatatype::Smallint(len, SqlSigned::Signed)
            | SqlDatatype::Mediumint(len, SqlSigned::Signed)
            | SqlDatatype::Tinyint(len, SqlSigned::Signed) => {
                RustDatatype(RUST_TYPE_I32.to_string(), Some(*len))
            }
            SqlDatatype::Float(_, fp, _) => RustDatatype(RUST_TYPE_F32.to_string(), Some(*fp)),
            SqlDatatype::Double(_, fp, _) | SqlDatatype::Decimal(_, fp, _) => {
                RustDatatype(RUST_TYPE_F64.to_string(), Some(*fp))
            }
            SqlDatatype::Bigint(len, SqlSigned::Unsigned) => {
                RustDatatype(RUST_TYPE_U64.to_string(), Some(*len))
            }
            SqlDatatype::Bigint(len, SqlSigned::Signed) => {
                RustDatatype(RUST_TYPE_I64.to_string(), Some(*len))
            }
            SqlDatatype::Date => RustDatatype(RUST_TYPE_STRING.to_string(), Some(10)),
            SqlDatatype::Time => RustDatatype(RUST_TYPE_STRING.to_string(), Some(8)),
            SqlDatatype::Datetime | SqlDatatype::Timestamp => {
                RustDatatype(RUST_TYPE_STRING.to_string(), Some(20))
            }
            SqlDatatype::Char(length)
            | SqlDatatype::Varchar(length)
            | SqlDatatype::Text(length) => {
                RustDatatype(RUST_TYPE_STRING.to_string(), Some(*length))
            }
            SqlDatatype::Binary(length) | SqlDatatype::Varbinary(length) => {
                RustDatatype(RUST_TYPE_STRING.to_string(), Some(*length))
            }
            SqlDatatype::Enum(options) => {
                RustDatatype(RUST_TYPE_STRING.to_string(), Some(options.iter().fold(0u32, |ac, c| if c.len() as u32 > ac { c.len()  as u32 } else { ac })))
            }
            SqlDatatype::Set(options) => {
                RustDatatype(RUST_TYPE_VEC.to_string(), Some(options.len() as u32))
            }
        }
    }
}
