use crate::alt_connection::Connection;
use crate::bisq::payload::*;
use actix::{Actor, Addr, AsyncContext, Context, Handler, StreamHandler};
use std::{io, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    socket: SocketAddr,
}
pub fn start(addr: NodeAddress) -> Addr<Server> {
    Server {
        socket: addr.into(),
    }
    .start()
}
impl Actor for Server {
    type Context = Context<Server>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let tcp = TcpListener::bind(&self.socket).expect("Unable to bind port");
        ctx.add_stream(tcp.incoming());
    }
}
impl StreamHandler<TcpStream, io::Error> for Server {
    fn handle(&mut self, connection: TcpStream, _ctx: &mut Self::Context) {
        // Connection::from_tcp_stream(connection, encode)
        // connection
        //     .set_nodelay(true)
        //     .expect("Unable to set nodelayx");

        // let (read, write) = connection.split();
        // let addr: Addr<Syn, _> = Connection::create(move |ctx| {
        //     let writer = Writer::new(write, ctx);
        //     Connection { writer }
        // });

        // let stream = ReadHalfStream { socket: read };

        // addr.do_send(AttachReadStream { stream });
    }
}
