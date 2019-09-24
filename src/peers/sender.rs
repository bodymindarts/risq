use crate::bisq::payload::*;
use crate::connection::Connection;
use crate::error::Error;
use actix::{Actor, Addr, Arbiter, Context, Handler, Message};
use tokio::{
    prelude::{
        future::{self, Future, IntoFuture, Loop},
        stream::Stream,
        Sink,
    },
    sync::mpsc,
};

enum ChannelMessage {
    SendPayload(network_envelope::Message),
}
pub struct Sender {
    sender: mpsc::Sender<ChannelMessage>,
}
impl Sender {
    pub fn start(conn: Connection) -> Addr<Sender> {
        let (sender, rec) = mpsc::channel(10);
        Arbiter::spawn(
            future::loop_fn((rec, conn), |(rec, conn)| {
                rec.into_future()
                    .map_err(|(e, _)| e.into())
                    .and_then(|(msg, rec)| msg.ok_or(Error::ReceiveMPSCError).map(|msg| (msg, rec)))
                    .and_then(|(msg, rec)| match msg {
                        ChannelMessage::SendPayload(payload) => {
                            conn.send(payload).then(|conn| match conn {
                                Ok(conn) => Ok(Loop::Continue((rec, conn))),
                                Err(e) => Ok(Loop::Break(e)),
                            })
                        }
                    })
                    .map_err(|e| error!("Sender errored: {:?}", e))
            })
            .map(|_| ()),
        );
        Sender { sender }.start()
    }
}
impl Actor for Sender {
    type Context = Context<Sender>;
}

pub struct SendPayload(pub network_envelope::Message);
impl Message for SendPayload {
    type Result = Result<(), Error>;
}
impl Handler<SendPayload> for Sender {
    type Result = Box<dyn Future<Item = (), Error = Error>>;
    fn handle(&mut self, msg: SendPayload, _: &mut Self::Context) -> Self::Result {
        Box::new(
            self.sender
                .clone()
                .sink_from_err::<Error>()
                .send(ChannelMessage::SendPayload(msg.0))
                .map(|_| ()),
        )
    }
}
