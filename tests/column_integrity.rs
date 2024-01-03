use database_reflection::metadata::consts::{
    METADATA_CHARSET, METADATA_COLLATION, METADATA_FLAG_AUTO_INCREMENT, METADATA_FLAG_NULLABLE,
    METADATA_FLAG_PRIMARY, METADATA_FLAG_UNSIGNED,
};
use database_reflection::metadata::WithMetadata;
use database_reflection::reflection::{
    Column, DefaultValue, JsonDatatype, JsonNumber, JsonString, RustDatatype, SqlDatatype,
    SqlSigned,
};
use serde_json::Value;

#[test]
fn test_column_integrity() {
    let mut column_pk = Column::new("test", "id", SqlDatatype::Int(32, SqlSigned::Unsigned));
    column_pk.set_meta_flag(METADATA_FLAG_UNSIGNED);
    column_pk.set_meta_flag(METADATA_FLAG_PRIMARY);
    column_pk.set_meta_flag(METADATA_FLAG_AUTO_INCREMENT);

    assert_eq!(column_pk.name(), String::from("id").into());
    assert_eq!(column_pk.table(), String::from("test").into());
    assert_eq!(
        column_pk.datatype(),
        &SqlDatatype::Int(32, SqlSigned::Unsigned)
    );
    assert_eq!(
        column_pk.datatype_json(),
        &JsonDatatype::Number(JsonNumber::Int)
    );
    assert_eq!(
        column_pk.datatype_rust(),
        &RustDatatype("u32".to_string(), Some(32))
    );
    assert_eq!(column_pk.default(), None);
    assert!(column_pk.meta_flag(METADATA_FLAG_UNSIGNED));
    assert!(column_pk.meta_flag(METADATA_FLAG_PRIMARY));
    assert!(column_pk.meta_flag(METADATA_FLAG_AUTO_INCREMENT));

    let mut column_vc = Column::new("test", "value", SqlDatatype::Varchar(64));
    column_vc.set_default(Some(DefaultValue::Value(Value::String(
        "empty".to_string(),
    ))));
    column_vc.set_meta(METADATA_COLLATION, "utf8mb4");
    column_vc.set_meta(METADATA_CHARSET, "utf8mb4_unicode_ci");

    assert_eq!(column_vc.name(), String::from("value").into());
    assert_eq!(column_vc.table(), String::from("test").into());
    assert_eq!(column_vc.datatype(), &SqlDatatype::Varchar(64));
    assert_eq!(
        column_vc.datatype_json(),
        &JsonDatatype::String(JsonString::String, Some(64))
    );
    assert_eq!(
        column_vc.datatype_rust(),
        &RustDatatype("String".to_string(), Some(64))
    );
    assert_eq!(
        column_vc.default(),
        Some(DefaultValue::Value(Value::String("empty".to_string())))
    );
    assert_eq!(
        column_vc.meta(METADATA_COLLATION),
        Some(String::from("utf8mb4"))
    );
    assert_eq!(
        column_vc.meta(METADATA_CHARSET),
        Some(String::from("utf8mb4_unicode_ci"))
    );
    assert!(!column_vc.meta_flag(METADATA_FLAG_NULLABLE));
}
