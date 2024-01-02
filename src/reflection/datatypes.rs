use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Basic SQL datatypes, defaults to VARCHAR(45)
pub enum SqlDatatype {
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
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDatatypeError;

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
                "float" | "real" => {
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
                        Ok(SqlDatatype::Float(left, right))
                    } else {
                        Ok(SqlDatatype::Real(left, right))
                    }
                }
                _ => {
                    if let Ok(len_val) = type_length.parse::<u32>() {
                        match type_group {
                            "tinyint" => Ok(SqlDatatype::Tinyint(len_val)),
                            "int" => Ok(SqlDatatype::Int(len_val)),
                            "bigint" => Ok(SqlDatatype::Bigint(len_val)),

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
    Float
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
    Json
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Basic JSON datatypes, defaults to string
pub enum JsonDatatype {
    Number(JsonNumber),
    String(JsonString, Option<u32>),
    Boolean,
    Array(Vec<String>),
    Object(Value)
}

impl Default for JsonDatatype {
    fn default() -> Self {
        JsonDatatype::String(JsonString::String, None)
    }
}

impl From<&SqlDatatype> for JsonDatatype {
    fn from(value: &SqlDatatype) -> Self {
        match value {
            SqlDatatype::Tinyint(1) => JsonDatatype::Boolean,
            SqlDatatype::Int(_) | SqlDatatype::Tinyint(_) => JsonDatatype::Number(JsonNumber::Int),
            SqlDatatype::Float(_,_) | SqlDatatype::Real(_,_) => JsonDatatype::Number(JsonNumber::Float),
            SqlDatatype::Bigint(_) => JsonDatatype::Number(JsonNumber::BigInt),
            SqlDatatype::Date => JsonDatatype::String(JsonString::Date, Some(10)),
            SqlDatatype::Time => JsonDatatype::String(JsonString::Time, Some(8)),
            SqlDatatype::Datetime | SqlDatatype::Timestamp => JsonDatatype::String(JsonString::Datetime, Some(20)),
            SqlDatatype::Char(length) | SqlDatatype::Varchar(length) | SqlDatatype::Text(length) => JsonDatatype::String(JsonString::String, Some(length.clone())),
            SqlDatatype::Binary(length) | SqlDatatype::Varbinary(length) => JsonDatatype::String(JsonString::String, Some(length.clone())),
            SqlDatatype::Enum(options) | SqlDatatype::Set(options) => JsonDatatype::Array(options.clone())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Basic Rust type as string
pub struct RustType(String, Option<u32>);

pub const RUST_TYPE_STRING:&str = "String";
pub const RUST_TYPE_USIZE:&str = "usize";
pub const RUST_TYPE_U32:&str = "u32";
pub const RUST_TYPE_F32:&str = "f32";
pub const RUST_TYPE_BOOL:&str = "bool";
pub const RUST_TYPE_VEC:&str = "Vec<String>";

impl Default for RustType {
    fn default() -> Self {
        RustType(RUST_TYPE_STRING.to_string(), None)
    }
}

impl From<&SqlDatatype> for RustType {

    fn from(value: &SqlDatatype) -> Self {
        match value {
            SqlDatatype::Tinyint(1) => RustType(RUST_TYPE_BOOL.to_string(), None),
            SqlDatatype::Int(len) | SqlDatatype::Tinyint(len) => RustType(RUST_TYPE_U32.to_string(), Some(len.clone())),
            SqlDatatype::Float(_,fp) | SqlDatatype::Real(_,fp) => RustType(RUST_TYPE_F32.to_string(), Some(fp.clone())),
            SqlDatatype::Bigint(len) => RustType(RUST_TYPE_USIZE.to_string(), Some(len.clone())),
            SqlDatatype::Date => RustType(RUST_TYPE_STRING.to_string(), Some(10)),
            SqlDatatype::Time => RustType(RUST_TYPE_STRING.to_string(), Some(8)),
            SqlDatatype::Datetime | SqlDatatype::Timestamp => RustType(RUST_TYPE_STRING.to_string(), Some(20)),
            SqlDatatype::Char(length) | SqlDatatype::Varchar(length) | SqlDatatype::Text(length) => RustType(RUST_TYPE_STRING.to_string(), Some(length.clone())),
            SqlDatatype::Binary(length) | SqlDatatype::Varbinary(length) => RustType(RUST_TYPE_STRING.to_string(), Some(length.clone())),
            SqlDatatype::Enum(options) | SqlDatatype::Set(options) => RustType(RUST_TYPE_VEC.to_string(), Some(options.len() as u32)),
        }
    }
}