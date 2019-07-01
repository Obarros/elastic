use crate::client::{
    receiver::parse,
    responses::*,
};

#[test]
fn success_parse_found_response() {
    let f = include_bytes!("delete_found.json");
    let deserialized = parse::<DeleteResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(8), deserialized.version());

    assert!(deserialized.deleted());
}

#[test]
fn success_parse_not_found_response() {
    let f = include_bytes!("delete_not_found.json");
    let deserialized = parse::<DeleteResponse>()
        .from_slice(StatusCode::NOT_FOUND, f as &[_])
        .unwrap();

    assert!(!deserialized.deleted());
}
