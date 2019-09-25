use super::{
    message,
    sender::{SendPayload, Sender},
    Peers,
};
use crate::bisq::{constants, payload::*};
use crate::connection::MessageStream;
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{Addr, Arbiter, WeakAddr};
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

struct ReportedPeersResponder {
    peers: Addr<Peers>,
    return_addr: WeakAddr<Sender>,
}
impl Listener for ReportedPeersResponder {
    fn get_peers_request(&mut self, msg: &GetPeersRequest) -> Accept {
        if let Some(addr) = self.return_addr.upgrade() {
            let request_nonce = msg.nonce;
            Arbiter::spawn(
                self.peers
                    .send(message::GetReportedPeers {})
                    .and_then(move |reported_peers| {
                        let res = GetPeersResponse {
                            request_nonce,
                            reported_peers,
                            supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                        };
                        addr.send(SendPayload(res.into()))
                    })
                    .then(|_| Ok(())),
            )
        }
        Accept::Processed
    }
}
pub fn listen(
    message_stream: MessageStream,
    return_addr: WeakAddr<Sender>,
    peers: Addr<Peers>,
) -> () {
    let listener = ReportedPeersResponder { peers, return_addr };

    Arbiter::spawn(
        future::loop_fn((listener, message_stream), |(mut listener, stream)| {
            stream
                .into_future()
                .map_err(|(e, _)| e)
                .and_then(|(msg, stream)| {
                    listener
                        .accept_or_err(&msg, Error::ConnectionClosed)
                        .map(|accepted| match accepted {
                            Accept::Processed => Loop::Continue((listener, stream)),
                            Accept::Skipped => {
                                warn!("Incoming listener skipped message: {:?}", msg);
                                Loop::Continue((listener, stream))
                            }
                        })
                })
        })
        .map_err(|e| info!("Connection closed: {:?}", e)),
    )
}
