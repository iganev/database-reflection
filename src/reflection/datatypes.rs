use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDatatypeError;

impl Default for Datatype {
    fn default() -> Self {
        Datatype::Varchar(45)
    }
}

impl FromStr for Datatype {
    type Err = ParseDatatypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Datatype::try_from(s)
    }
}

impl TryFrom<&str> for Datatype {
    type Error = ParseDatatypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        //TODO

        if value.starts_with("int") {
            return Ok(Datatype::Int(10))
        } else if value.starts_with("varchar") {
            return Ok(Datatype::Varchar(45))
        } else if value.starts_with("tinyint") {
            return Ok(Datatype::Tinyint(1))
        }

        Ok(Datatype::Int(10))
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultValue {
    #[default]
    Null,
    Value(Value),
}
