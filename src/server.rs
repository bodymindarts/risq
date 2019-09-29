use crate::bisq::payload::NodeAddress;
use crate::bootstrap::Bootstrap;
use crate::peers::Peers;
use crate::tor::{AddOnionConfig, TorControl};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, StreamHandler};
use std::io;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::future::Future,
};

pub struct TorConf {
    pub hidden_service_port: u16,
    pub tc_port: u16,
    pub private_key_path: PathBuf,
}

pub struct Server {
    listen_port: u16,
    tor_conf: Option<TorConf>,
    peers: Addr<Peers>,
    bootstrap: Addr<Bootstrap>,
}
pub fn start(
    listen_port: u16,
    peers: Addr<Peers>,
    bootstrap: Addr<Bootstrap>,
    tor_conf: Option<TorConf>,
) -> Addr<Server> {
    Server {
        listen_port,
        tor_conf,
        peers,
        bootstrap,
    }
    .start()
}
impl Actor for Server {
    type Context = Context<Server>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let listen_socket =
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.listen_port);
        let tcp = TcpListener::bind(&listen_socket).expect("Unable to bind port");
        ctx.add_stream(tcp.incoming());
        let addr = match &self.tor_conf {
            Some(tor_conf) => {
                let mut tc = TorControl::connect(("localhost", tor_conf.tc_port))
                    .expect("Couldn't authenticate to TorControl");
                let onion_addr = tc
                    .add_v2_onion(AddOnionConfig {
                        virtual_port: tor_conf.hidden_service_port,
                        target_port: self.listen_port,
                        private_key_path: tor_conf.private_key_path.clone(),
                    })
                    .expect("Couldn't create hidden service");
                NodeAddress {
                    host_name: onion_addr.onion_service,
                    port: onion_addr.port as i32,
                }
            }
            None => NodeAddress {
                host_name: "localhost".to_string(),
                port: self.listen_port as i32,
            },
        };

        info!("Server started @ {:?}", addr);
        Arbiter::spawn(
            self.bootstrap
                .send(event::ServerStarted(addr.clone()))
                .then(|_| Ok(())),
        );
        Arbiter::spawn(self.peers.send(event::ServerStarted(addr)).then(|_| Ok(())));
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
