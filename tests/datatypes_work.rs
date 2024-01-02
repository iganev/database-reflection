use database_reflection::reflection::{DefaultValue, ParseDatatypeError, SqlDatatype};
use serde_json::Value;
use std::str::FromStr;

#[test]
fn test_datatypes() {
    assert_eq!(
        SqlDatatype::try_from("tinyint(1)"),
        Ok(SqlDatatype::Tinyint(1))
    );
    assert_eq!(SqlDatatype::try_from("int(10)"), Ok(SqlDatatype::Int(10)));
    assert_eq!(
        SqlDatatype::try_from("bigint(32)"),
        Ok(SqlDatatype::Bigint(32))
    );
    assert_eq!(
        SqlDatatype::try_from("float(4,2)"),
        Ok(SqlDatatype::Float(4, 2))
    );
    assert_eq!(
        SqlDatatype::try_from("real(10,2)"),
        Ok(SqlDatatype::Real(10, 2))
    );

    assert_eq!(SqlDatatype::try_from("date"), Ok(SqlDatatype::Date));
    assert_eq!(SqlDatatype::try_from("time"), Ok(SqlDatatype::Time));
    assert_eq!(SqlDatatype::try_from("datetime"), Ok(SqlDatatype::Datetime));
    assert_eq!(
        SqlDatatype::try_from("timestamp"),
        Ok(SqlDatatype::Timestamp)
    );

    assert_eq!(SqlDatatype::try_from("char(64)"), Ok(SqlDatatype::Char(64)));
    assert_eq!(
        SqlDatatype::try_from("varchar(45)"),
        Ok(SqlDatatype::Varchar(45))
    );
    assert_eq!(
        SqlDatatype::from_str("varchar(64)"),
        Ok(SqlDatatype::Varchar(64))
    );
    assert_eq!(
        SqlDatatype::try_from("text(1024)"),
        Ok(SqlDatatype::Text(1024))
    );
    assert_eq!(SqlDatatype::try_from("text"), Ok(SqlDatatype::Text(65535)));

    assert_eq!(
        SqlDatatype::try_from("binary(32)"),
        Ok(SqlDatatype::Binary(32))
    );
    assert_eq!(
        SqlDatatype::try_from("varbinary(32)"),
        Ok(SqlDatatype::Varbinary(32))
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
        SqlDatatype::try_from(r#"set("this","that")"#),
        Ok(SqlDatatype::Set(vec![
            String::from("this"),
            String::from("that")
        ]))
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
