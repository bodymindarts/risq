use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::{GetDataResponse, Listener, PreliminaryGetDataRequest},
};
use crate::connection::{Connection, ConnectionConfig};
use crate::error::Error;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tokio::prelude::{
    future::{self, Future, IntoFuture, Loop},
    stream::Stream,
};

pub struct Config {
    network: BaseCurrencyNetwork,
}
pub struct BootstrapResult {}
struct GetDataResponseCollector {
    expecting_nonce: i32,
    response: Option<GetDataResponse>,
}
impl Listener for GetDataResponseCollector {
    fn get_data_response(&mut self, response: GetDataResponse) {
        if response.request_nonce == self.expecting_nonce {
            self.response = Some(response)
        }
    }
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: thread_rng().gen(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    let conn = Connection::new(
        addr,
        ConnectionConfig {
            message_version: config.network.into(),
        },
    )
    .and_then(|mut conn| {
        let listener = GetDataResponseCollector {
            expecting_nonce: preliminary_get_data_request.nonce,
            response: None,
        };
        conn.send(preliminary_get_data_request)
            .and_then(|mut conn| {
                future::loop_fn(
                    (listener, conn.take_message_stream()),
                    |(mut listener, stream)| {
                        stream.into_future().map_err(|(err, _stream)| err).and_then(
                            |(msg, stream)| {
                                listener
                                    .accept_or_err(msg, Error::DidNotReceiveExpectedResponse)
                                    .and_then(|_| {
                                        if let Some(response) = listener.response {
                                            Ok(Loop::Break(response))
                                        } else {
                                            Ok(Loop::Continue((listener, stream)))
                                        }
                                    })
                            },
                        )
                    },
                )
            })
    });
    future::ok(BootstrapResult {})
}
