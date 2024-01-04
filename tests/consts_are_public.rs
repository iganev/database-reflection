use database_reflection::metadata::consts::*;

#[test]
fn test_consts_are_public() {
    assert!(METADATA_CHARSET.len() > 0);
    assert!(METADATA_COLLATION.len() > 0);
    assert!(METADATA_ON_UPDATE.len() > 0);
    assert!(METADATA_ON_DELETE.len() > 0);
    assert!(METADATA_CASCADE.len() > 0);
    assert!(METADATA_SET_NULL.len() > 0);

    assert!(METADATA_FLAG_UNSIGNED.len() > 0);
    assert!(METADATA_FLAG_NULLABLE.len() > 0);
    assert!(METADATA_FLAG_PRIMARY.len() > 0);
    assert!(METADATA_FLAG_UNIQUE.len() > 0);
    assert!(METADATA_FLAG_AUTO_INCREMENT.len() > 0);
    assert!(METADATA_FLAG_DEFAULT_CURRENT_TIMESTAMP.len() > 0);
    assert!(METADATA_FLAG_ON_UPDATE_CURRENT_TIMESTAMP.len() > 0);
}
