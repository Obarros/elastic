/*!
Builders for raw requests.
*/

use fluent_builder::TryIntoValue;
use std::marker::PhantomData;

use client::{
    requests::{
        Endpoint,
        RequestBuilder,
    },
    sender::{
        NextParams,
        NodeAddresses,
        SendableRequest,
        SendableRequestParams,
        Sender,
    },
    Client,
};

/**
A raw request builder that can be configured before sending.

Call [`Client.request`][Client.request] to get an `IndexRequest`.
The `send` method will either send the request synchronously or asynchronously, depending on the `Client` it was created from.

[Client.request]: ../../struct.Client.html#raw-request
*/
pub type RawRequestBuilder<TSender, TEndpoint, TBody> =
    RequestBuilder<TSender, RawRequestInner<TEndpoint, TBody>>;

#[doc(hidden)]
pub struct RawRequestInner<TEndpoint, TBody> {
    endpoint: TEndpoint,
    _marker: PhantomData<TBody>,
}

impl<TEndpoint, TBody> RawRequestInner<TEndpoint, TBody> {
    pub(crate) fn new(endpoint: TEndpoint) -> Self {
        RawRequestInner {
            endpoint,
            _marker: PhantomData,
        }
    }
}

/**
# Raw request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /**
    Create a [`RawRequestBuilder`][RawRequestBuilder] with this `Client` that can be configured before sending.

    The `request` method accepts any type that can be converted into a [`Endpoint<'static>`][Endpoint],
    which includes the endpoint types in the [`endpoints`][endpoints-mod] module.

    # Examples

    Send a cluster ping and read the returned metadata:

    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    // `PingRequest` implements `Into<Endpoint>`
    let req = PingRequest::new();

    // Turn the `PingRequest` into a `RequestBuilder`
    let builder = client.request(req);

    // Send the `RequestBuilder` and parse as a `PingResponse`
    let ping = builder.send()?.into_response::<PingResponse>()?;

    println!("cluster: {}", ping.name());
    # Ok(())
    # }
    ```

    [Endpoint]: requests/struct.Endpoint.html
    [RawRequestBuilder]: requests/raw/type.RawRequestBuilder.html
    [endpoints-mod]: requests/endpoints/index.html
    */
    pub fn request<TEndpoint, TBody>(
        &self,
        endpoint: TEndpoint,
    ) -> RawRequestBuilder<TSender, TEndpoint, TBody>
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<TSender::Body>,
    {
        RequestBuilder::initial(self.clone(), RawRequestInner::new(endpoint))
    }
}

impl<TSender, TEndpoint, TBody> RawRequestBuilder<TSender, TEndpoint, TBody>
where
    TSender: Sender,
    TEndpoint: Into<Endpoint<'static, TBody>>,
    TBody: Into<<TSender>::Body> + Send + 'static,
    NodeAddresses<TSender>: NextParams,
    <NodeAddresses<TSender> as NextParams>::Params: Into<TSender::Params> + Send + 'static,
{
    /**
    Send a `RawRequestBuilder`.

    If this request is for a [`SyncClient`][SyncClient], then `send` will block the current thread until a response arrives and is deserialised.
    The returned [`SyncResponseBuilder`][SyncResponseBuilder] can be used to parse the response.

    If this request is for an [`AsyncClient`][AsyncClient], then `send` will return a future that will resolve to the deserialised index response.
    The returned [`AsyncResponseBuilder`][AsyncResponseBuilder] can be used to parse the response.

    # Examples

    Send a raw request synchronously and parse it to a concrete response type:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                         .send()?
                         .into_response::<SearchResponse<Value>>()?;

    // Iterate through the hits (of type `MyType`)
    for hit in response.hits() {
        println!("{:?}", hit);
    }
    # Ok(())
    # }
    ```

    Send a raw request asynchronously and parse it to a concrete response type:

    ```no_run
    # extern crate tokio;
    # extern crate futures;
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # use futures::Future;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                       .send()
                       .and_then(|res| res.into_response::<SearchResponse<Value>>());

    future.and_then(|response| {
        // Iterate through the hits (of type `MyType`)
        for hit in response.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    [SyncResponseBuilder]: ../../responses/struct.SyncResponseBuilder.html
    [AsyncClient]: ../../type.AsyncClient.html
    [AsyncResponseBuilder]: ../../responses/struct.AsyncResponseBuilder.html
    */
    pub fn send(self) -> TSender::Response {
        let client = self.client;
        let endpoint = self.inner.endpoint.into();

        // Only try fetch a next address if an explicit `RequestParams` hasn't been given
        let params = match self.params_builder.try_into_value() {
            TryIntoValue::Value(value) => SendableRequestParams::Value(value),
            TryIntoValue::Builder(builder) => SendableRequestParams::Builder {
                params: client.addresses.next(),
                builder,
            },
        };

        let req = SendableRequest::new(endpoint, params);

        client.sender.send(req)
    }
}
