// use super::{
//     keep_alive::KeepAliveListener,
//     message,
//     sender::{SendPayload, Sender},
//     Peers, ReportedPeersListener,
// };
// use crate::bisq::{constants, payload::*};
// use crate::connection::{ConnectionId, MessageStream};
// use crate::error::Error;
// use crate::listener::{chain, Accept, Listener};
// use actix::{Addr, Arbiter, WeakAddr};
// use tokio::prelude::{
//     future::{self, Future, Loop},
//     stream::Stream,
// };

// pub fn listen(
//     message_stream: MessageStream,
//     return_addr: WeakAddr<Sender>,
//     peers: Addr<Peers>,
// ) -> () {
//     let listener = chain(ReportedPeersListener {
//         peers,
//         from: message_stream.id,
//     })
//     .forward_to(KeepAliveListener { return_addr });

//     Arbiter::spawn(
//         future::loop_fn((listener, message_stream), |(mut listener, stream)| {
//             stream
//                 .into_future()
//                 .map_err(|(e, _)| e)
//                 .and_then(|(msg, stream)| {
//                     listener
//                         .accept_or_err(&msg, Error::ConnectionClosed)
//                         .map(|accepted| match accepted {
//                             Accept::Processed => Loop::Continue((listener, stream)),
//                             Accept::Skipped => {
//                                 warn!("Incoming listener skipped message: {:?}", msg);
//                                 Loop::Continue((listener, stream))
//                             }
//                         })
//                 })
//         })
//         .map_err(|e| info!("Connection closed: {:?}", e)),
//     )
// }
