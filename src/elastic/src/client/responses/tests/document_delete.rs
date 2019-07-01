use super::load_file;
use crate::{
    client::{
        receiver::{
            parse,
            ResponseError,
        },
        responses::*,
    },
    error::*,
};

#[test]
fn success_parse_found_response() {
    let f = load_file("delete_found.json");
    let deserialized = parse::<DeleteResponse>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(8), deserialized.version());

    assert!(deserialized.deleted());
}

#[test]
fn success_parse_not_found_response() {
    let f = load_file("delete_not_found.json");
    let deserialized = parse::<DeleteResponse>()
        .from_reader(StatusCode::NOT_FOUND, f)
        .unwrap();

    assert!(!deserialized.deleted());
}
