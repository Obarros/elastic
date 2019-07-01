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
fn success_parse_ping_response() {
    let f = load_file("ping_success.json");
    let deserialized = parse::<PingResponse>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!("Scorcher", deserialized.name());
}
