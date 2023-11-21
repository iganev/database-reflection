use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Basic SQL datatypes, defaults to VARCHAR(45)
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
                        Ok(Datatype::Set(options))
                    } else {
                        Ok(Datatype::Enum(options))
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
                        Ok(Datatype::Float(left, right))
                    } else {
                        Ok(Datatype::Real(left, right))
                    }
                }
                _ => {
                    if let Ok(len_val) = type_length.parse::<u32>() {
                        match type_group {
                            "tinyint" => Ok(Datatype::Tinyint(len_val)),
                            "int" => Ok(Datatype::Int(len_val)),
                            "bigint" => Ok(Datatype::Bigint(len_val)),

                            "char" => Ok(Datatype::Char(len_val)),
                            "varchar" => Ok(Datatype::Varchar(len_val)),
                            "text" => Ok(Datatype::Text(len_val)), // can be without length

                            "binary" => Ok(Datatype::Binary(len_val)),
                            "varbinary" => Ok(Datatype::Varbinary(len_val)),

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
                "text" => Ok(Datatype::Text(65535)),
                "date" => Ok(Datatype::Date),
                "time" => Ok(Datatype::Time),
                "datetime" => Ok(Datatype::Datetime),
                "timestamp" => Ok(Datatype::Timestamp),
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
