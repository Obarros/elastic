/*!
Elasticsearch Response Iterators

A crate to handle parsing and handling Elasticsearch search results which provides
convenient iterators to step through the results returned. It is designed to work
with [`elastic-reqwest`][elastic-reqwest].
It also re-exports `serde_json::Value` for convenient anonymous json objects.

This crate provides parsers that can be used to convert a http response into a concrete
type or an API error.

## Usage

This crate is on [crates.io][crates-io].
Add `elastic_responses` to your `Cargo.toml`:

```text
[dependencies]
elastic_responses = "~0.21.0-pre.4"
```

Use the [`parse`][parse] function to deserialise a http response to a `Result<T, ApiError>` for some
concrete response type `T`.

# Examples

Run a [Query DSL][query-dsl] query, then iterate through the results:

```no_run
# extern crate elastic_responses;
# use crate::client::responses::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# fn main() {
// Send a search request and read as a response
let (response_status, response_body) = do_request();

// Parse body to JSON as an crate::client::responses::SearchResponse object
// If the response is an API error then it'll be parsed into a friendly Rust error
let response = parse::<SearchResponse<Value>>().from_slice(response_status, response_body).unwrap();

// Iterate over hits. Could also use `documents`
for hit in response.hits() {
    let score = hit.score().unwrap_or(f32::default());
    let doc = hit.document();

    println!("score: {}", score);
    println!("doc: {:?}", doc);
}

// Agregations are flattened into individual stats metrics
for agg in response.aggs() {
    let min_ack_pkts = agg["min_ack_pkts_sent"].as_u64().unwrap();
    let max_ack_pkts = agg["max_ack_pkts_sent"].as_u64().unwrap();

    println!("min: {}, max: {}", min_ack_pkts, max_ack_pkts);
}
# }
```

Any type that implements `Deserialize` can be used as the document type in the search response:

```no_run
# #[macro_use] extern crate serde_derive;
# extern crate elastic_responses;
# use crate::client::responses::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# fn main() {
#[derive(Deserialize)]
struct MyDocument {
    title: String,
    description: String
}

let (response_status, response_body) = do_request();

let response = parse::<SearchResponse<MyDocument>>().from_slice(response_status, response_body).unwrap();

for doc in response.documents() {
    println!("title: {}", doc.title);
    println!("description: {}", doc.description);
}
# }
```

Run a [Get Document][get-document] request, and handle cases where the document wasn't found or the index doesn't exist:

```no_run
# #[macro_use] extern crate serde_json;
# extern crate elastic_responses;
# use serde_json::Value;
# use crate::client::responses::*;
# use crate::client::responses::error::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# fn main() {
// Send a document get request and read as a response
let (response_status, response_body) = do_request();

let response = parse::<GetResponse<Value>>().from_slice(response_status, response_body);

match response.map(|res| res.into_document()) {
    Ok(Some(doc)) => {
        // The document was found
    }
    Ok(None) => {
        // The document was not found
    }
    Err(ResponseError::Api(ApiError::IndexNotFound { index })) => {
        // The index doesn't exist
    }
    _ => {
        // Some other error
    }
}
# }
```

As with `SearchResponse`, any type that implements `Deserialize` can be used as the generic document type
in a `GetResponse`.

[elastic-reqwest]: https://github.com/elastic-rs/elastic-reqwest/
[crates-io]: https://crates.io/crates/elastic_responses
[query-dsl]: https://www.elastic.co/guide/en/elasticsearch/reference/master/search.html
[get-document]: https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html
[parse]: parsing/fn.parse.html
*/

pub mod bulk;
mod command;
pub mod common;
mod document_delete;
mod document_get;
mod document_index;
mod document_update;
pub mod nodes_info;
mod ping;
pub mod search;
mod sql;

mod index_exists;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use self::{
    bulk::{
        BulkErrorsResponse,
        BulkResponse,
    },
    command::*,
    document_delete::*,
    document_get::*,
    document_index::*,
    document_update::*,
    nodes_info::NodesInfoResponse,
    ping::*,
    search::SearchResponse,
    sql::*,
};

pub use self::index_exists::*;

/** Re-export of `serde_json::Value` for convenience. */
pub use serde_json::Value;

pub use http::StatusCode;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{
        bulk::Action as BulkAction,
        BulkErrorsResponse,
        BulkResponse,
        CommandResponse,
        DeleteResponse,
        GetResponse,
        IndexResponse,
        IndicesExistsResponse,
        NodesInfoResponse,
        PingResponse,
        SearchResponse,
        SqlResponse,
        UpdateResponse,
    };
}
