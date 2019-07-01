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
fn success_parse_command_response() {
    let f = load_file("acknowledged.json");
    let deserialized = parse::<CommandResponse>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert!(deserialized.acknowledged());
}
