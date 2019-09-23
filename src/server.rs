use crate::bisq::message::*;
use crate::connection::Connection;
use crate::error::Error;
use crate::peers::Peers;
use actix::Addr;
use std::net::ToSocketAddrs;
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::{
        future::{self, Future, IntoFuture, Loop},
        stream::Stream,
        Sink,
    },
    sync::oneshot,
};

pub fn start(
    addr: NodeAddress,
    message_version: MessageVersion,
    started: oneshot::Sender<NodeAddress>,
    opened: Addr<Peers>,
) -> impl Future<Item = (), Error = Error> {
    let socket = addr.clone().into();
    info!("Starting server listening to: {:?}", socket);
    TcpListener::bind(&socket)
        .map_err(|e| e.into())
        .and_then(|server| {
            started
                .send(addr)
                .map(|_| server)
                .map_err(|_| Error::SendOneshotError)
        })
        .into_future()
        .and_then(move |server| {
            future::loop_fn((opened, server.incoming()), move |(opened, stream)| {
                stream
                    .into_future()
                    .map_err(|(e, _)| e.into())
                    .and_then(move |(socket, stream)| {
                        socket
                            .ok_or(Error::ServerShutdown)
                            .into_future()
                            .and_then(move |socket| {
                                debug!("New connection received {:?}", socket);
                                opened
                                    .send(Connection::from_tcp_stream(
                                        socket,
                                        message_version.clone(),
                                    ))
                                    .map_err(|e| e.into())
                                    .map(|_| Loop::Continue((opened, stream)))
                            })
                    })
            })
        })
}
