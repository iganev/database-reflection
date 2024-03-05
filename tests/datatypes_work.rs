use database_reflection::reflection::{
    DefaultValue, JsonDatatype, JsonNumber, JsonString, ParseDatatypeError, RustDatatype,
    SqlDatatype, SqlSigned,
};
use serde_json::Value;
use std::str::FromStr;

#[test]
fn test_datatypes() {
    assert_eq!(
        SqlDatatype::try_from("tinyint(1) unsigned"),
        Ok(SqlDatatype::Tinyint(1, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("tinyint(1) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("tinyint(1) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(1)
    );
    assert_eq!(
        SqlDatatype::try_from("int(10) unsigned"),
        Ok(SqlDatatype::Int(10, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("int(10)"),
        Ok(SqlDatatype::Int(10, SqlSigned::Signed))
    );
    assert_eq!(
        SqlDatatype::try_from("int(10) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("int(10) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(10)
    );
    assert_eq!(
        SqlDatatype::try_from("int(10) unsigned")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("u32".to_string(), Some(10)))
    );
    assert_eq!(
        SqlDatatype::try_from("int(10)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("i32".to_string(), Some(10)))
    );
    assert_eq!(
        SqlDatatype::try_from("smallint(5) unsigned"),
        Ok(SqlDatatype::Smallint(5, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("smallint(5) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("smallint(5) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(5)
    );
    assert_eq!(
        SqlDatatype::try_from("smallint(5)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("i32".to_string(), Some(5)))
    );
    assert_eq!(
        SqlDatatype::try_from("mediumint(15) unsigned"),
        Ok(SqlDatatype::Mediumint(15, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("mediumint(15) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("mediumint(15) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(15)
    );
    assert_eq!(
        SqlDatatype::try_from("mediumint(15)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("i32".to_string(), Some(15)))
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32)"),
        Ok(SqlDatatype::Bigint(32, SqlSigned::Signed))
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32)")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Signed)
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(32)
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::Number(JsonNumber::BigInt))
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32) unsigned")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("u64".to_string(), Some(32)))
    );
    assert_eq!(
        SqlDatatype::try_from("bigint(32)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("i64".to_string(), Some(32)))
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)"),
        Ok(SqlDatatype::Float(4, 2, SqlSigned::Signed))
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Signed)
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(4)
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::Number(JsonNumber::Float))
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("f32".to_string(), Some(2)))
    );
    assert_eq!(
        SqlDatatype::try_from("double(10,2) unsigned"),
        Ok(SqlDatatype::Double(10, 2, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("double(10,2) unsigned")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::Number(JsonNumber::Float))
    );
    assert_eq!(
        SqlDatatype::try_from("double(10,2) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("double(10,2) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(10)
    );
    assert_eq!(
        SqlDatatype::try_from("double(10,2)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("f64".to_string(), Some(2)))
    );
    assert_eq!(
        SqlDatatype::try_from("decimal(10,2) unsigned"),
        Ok(SqlDatatype::Decimal(10, 2, SqlSigned::Unsigned))
    );
    assert_eq!(
        SqlDatatype::try_from("decimal(10,2) unsigned")
            .ok()
            .map(|t| t.sign())
            .unwrap_or_default(),
        Some(SqlSigned::Unsigned)
    );
    assert_eq!(
        SqlDatatype::try_from("decimal(10,2) unsigned")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(10)
    );

    assert_eq!(SqlDatatype::try_from("date"), Ok(SqlDatatype::Date));
    assert_eq!(
        SqlDatatype::try_from("date")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::String(JsonString::Date, Some(10)))
    );
    assert_eq!(
        SqlDatatype::try_from("date")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(10)))
    );
    assert_eq!(SqlDatatype::try_from("time"), Ok(SqlDatatype::Time));
    assert_eq!(
        SqlDatatype::try_from("time")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::String(JsonString::Time, Some(8)))
    );
    assert_eq!(
        SqlDatatype::try_from("time")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(8)))
    );
    assert_eq!(SqlDatatype::try_from("datetime"), Ok(SqlDatatype::Datetime));
    assert_eq!(
        SqlDatatype::try_from("timestamp"),
        Ok(SqlDatatype::Timestamp)
    );
    assert_eq!(
        SqlDatatype::try_from("timestamp")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        None
    );

    assert_eq!(SqlDatatype::try_from("char(64)"), Ok(SqlDatatype::Char(64)));
    assert_eq!(
        SqlDatatype::try_from("char(64)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(64)
    );
    assert_eq!(
        SqlDatatype::try_from("char(64)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(64)))
    );
    assert_eq!(
        SqlDatatype::try_from("varchar(45)"),
        Ok(SqlDatatype::Varchar(45))
    );
    assert_eq!(
        SqlDatatype::from_str("varchar(64)"),
        Ok(SqlDatatype::Varchar(64))
    );
    assert_eq!(
        SqlDatatype::try_from("varchar(64)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(64)
    );
    assert_eq!(
        SqlDatatype::try_from("varchar(64)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::String(JsonString::String, Some(64)))
    );
    assert_eq!(
        SqlDatatype::try_from("varchar(64)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(64)))
    );
    assert_eq!(
        SqlDatatype::try_from("text(1024)"),
        Ok(SqlDatatype::Text(1024))
    );
    assert_eq!(
        SqlDatatype::try_from("text(1024)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(1024)))
    );
    assert_eq!(
        SqlDatatype::try_from("text(1024)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(1024)
    );
    assert_eq!(SqlDatatype::try_from("text"), Ok(SqlDatatype::Text(65535)));
    assert_eq!(
        SqlDatatype::try_from("text")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(65535)
    );

    assert_eq!(
        SqlDatatype::try_from("binary(32)"),
        Ok(SqlDatatype::Binary(32))
    );
    assert_eq!(
        SqlDatatype::try_from("binary(32)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(32)
    );
    assert_eq!(
        SqlDatatype::try_from("varbinary(32)"),
        Ok(SqlDatatype::Varbinary(32))
    );
    assert_eq!(
        SqlDatatype::try_from("varbinary(32)")
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(32)
    );
    assert_eq!(
        SqlDatatype::try_from("varbinary(32)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::String(JsonString::String, Some(32)))
    );
    assert_eq!(
        SqlDatatype::try_from("varbinary(32)")
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(32)))
    );
    assert_eq!(
        SqlDatatype::try_from(r#"enum("one","two","three")"#),
        Ok(SqlDatatype::Enum(vec![
            String::from("one"),
            String::from("two"),
            String::from("three")
        ]))
    );
    assert_eq!(
        SqlDatatype::try_from(r#"enum("one","two","three")"#)
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(3)
    );
    assert_eq!(
        SqlDatatype::try_from(r#"enum("one","two","three")"#)
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(JsonDatatype::Array(vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string()
        ]))
    );
    assert_eq!(
        SqlDatatype::try_from(r#"enum("one","two","three")"#)
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("String".to_string(), Some(5)))
    );
    assert_eq!(
        SqlDatatype::try_from(r#"set("this","that")"#),
        Ok(SqlDatatype::Set(vec![
            String::from("this"),
            String::from("that")
        ]))
    );
    assert_eq!(
        SqlDatatype::try_from(r#"set("this","that")"#)
            .ok()
            .map(|t| t.len())
            .unwrap_or_default(),
        Some(2)
    );
    assert_eq!(
        SqlDatatype::try_from(r#"set("this","that")"#)
            .ok()
            .map(|t| (&t).try_into())
            .unwrap(),
        Ok(RustDatatype("Vec<String>".to_string(), Some(2)))
    );

    assert_eq!(
        SqlDatatype::try_from("varchar(nan)"),
        Err(ParseDatatypeError)
    );
    assert_eq!(
        SqlDatatype::try_from("badtype(10)"),
        Err(ParseDatatypeError)
    );
    assert_eq!(SqlDatatype::try_from("badtype"), Err(ParseDatatypeError));

    //

    let data_val = DefaultValue::Value(Value::from("test"));
    let _ = DefaultValue::Null;

    if let DefaultValue::Value(Value::String(s)) = data_val {
        assert_eq!(s, "test".to_string());
    } else {
        assert!(false);
    }
}
