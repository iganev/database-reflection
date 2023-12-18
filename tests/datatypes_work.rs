use database_reflection::reflection::{Datatype, DefaultValue, ParseDatatypeError};
use serde_json::Value;
use std::str::FromStr;

#[test]
fn test_datatypes() {
    assert_eq!(Datatype::try_from("tinyint(1)"), Ok(Datatype::Tinyint(1)));
    assert_eq!(Datatype::try_from("int(10)"), Ok(Datatype::Int(10)));
    assert_eq!(Datatype::try_from("bigint(32)"), Ok(Datatype::Bigint(32)));
    assert_eq!(Datatype::try_from("float(4,2)"), Ok(Datatype::Float(4, 2)));
    assert_eq!(Datatype::try_from("real(10,2)"), Ok(Datatype::Real(10, 2)));

    assert_eq!(Datatype::try_from("date"), Ok(Datatype::Date));
    assert_eq!(Datatype::try_from("time"), Ok(Datatype::Time));
    assert_eq!(Datatype::try_from("datetime"), Ok(Datatype::Datetime));
    assert_eq!(Datatype::try_from("timestamp"), Ok(Datatype::Timestamp));

    assert_eq!(Datatype::try_from("char(64)"), Ok(Datatype::Char(64)));
    assert_eq!(Datatype::try_from("varchar(45)"), Ok(Datatype::Varchar(45)));
    assert_eq!(Datatype::from_str("varchar(64)"), Ok(Datatype::Varchar(64)));
    assert_eq!(Datatype::try_from("text(1024)"), Ok(Datatype::Text(1024)));
    assert_eq!(Datatype::try_from("text"), Ok(Datatype::Text(65535)));

    assert_eq!(Datatype::try_from("binary(32)"), Ok(Datatype::Binary(32)));
    assert_eq!(
        Datatype::try_from("varbinary(32)"),
        Ok(Datatype::Varbinary(32))
    );

    assert_eq!(
        Datatype::try_from(r#"enum("one","two","three")"#),
        Ok(Datatype::Enum(vec![
            String::from("one"),
            String::from("two"),
            String::from("three")
        ]))
    );
    assert_eq!(
        Datatype::try_from(r#"set("this","that")"#),
        Ok(Datatype::Set(vec![
            String::from("this"),
            String::from("that")
        ]))
    );

    assert_eq!(Datatype::try_from("varchar(nan)"), Err(ParseDatatypeError));
    assert_eq!(Datatype::try_from("badtype(10)"), Err(ParseDatatypeError));
    assert_eq!(Datatype::try_from("badtype"), Err(ParseDatatypeError));

    //

    let data_val = DefaultValue::Value(Value::from("test"));
    let _ = DefaultValue::Null;

    if let DefaultValue::Value(Value::String(s)) = data_val {
        assert_eq!(s, "test".to_string());
    } else {
        assert!(false);
    }
}
