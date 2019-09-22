use crate::error::Error;
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::{
        future::{self, Future, IntoFuture, Loop},
        stream::Stream,
        Sink,
    },
    sync::{mpsc, oneshot},
};

pub fn start(
    addr: SocketAddr,
    started: oneshot::Sender<()>,
    opened: mpsc::Sender<TcpStream>,
) -> impl Future<Item = (), Error = Error> {
    TcpListener::bind(&addr)
        .map_err(|e| e.into())
        .and_then(|server| {
            started
                .send(())
                .map(|_| server)
                .map_err(|_| Error::SendOneshotError)
        })
        .into_future()
        .and_then(|server| {
            future::loop_fn((opened, server.incoming()), |(opened, stream)| {
                stream
                    .into_future()
                    .map_err(|(e, _)| e.into())
                    .and_then(|(socket, stream)| {
                        socket
                            .ok_or(Error::ServerShutdown)
                            .into_future()
                            .and_then(|socket| {
                                debug!("New connection received {:?}", socket);
                                opened
                                    .send(socket)
                                    .map_err(|e| e.into())
                                    .map(|opened| Loop::Continue((opened, stream)))
                            })
                    })
            })
        })
}
