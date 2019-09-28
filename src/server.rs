use crate::bisq::payload::*;
use crate::peers::Peers;
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, StreamHandler};
use std::io;
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::future::Future,
};

pub struct Server {
    addr: NodeAddress,
    peers: Addr<Peers>,
}
pub fn start(addr: NodeAddress, peers: Addr<Peers>) -> Addr<Server> {
    Server { addr, peers }.start()
}
impl Actor for Server {
    type Context = Context<Server>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let tcp = TcpListener::bind(&self.addr.clone().into()).expect("Unable to bind port");
        ctx.add_stream(tcp.incoming());
        Arbiter::spawn(
            self.peers
                .send(event::ServerStarted(self.addr.clone()))
                .then(|_| Ok(())),
        );
    }
}
impl StreamHandler<TcpStream, io::Error> for Server {
    fn handle(&mut self, connection: TcpStream, _ctx: &mut Self::Context) {
        Arbiter::spawn(
            self.peers
                .send(event::IncomingConnection(connection))
                .then(|_| Ok(())),
        );
    }
}

pub mod event {
    use crate::bisq::payload::NodeAddress;
    use actix::Message;
    use tokio::net::TcpStream;

    pub struct ServerStarted(pub NodeAddress);
    impl Message for ServerStarted {
        type Result = ();
    }
    pub struct IncomingConnection(pub TcpStream);
    impl Message for IncomingConnection {
        type Result = ();
    }
}
