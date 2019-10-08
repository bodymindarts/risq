use super::{
    bootstrap::Bootstrap,
    dispatch::SendableDispatcher,
    peers::Peers,
    tor::{AddOnionConfig, TorControl},
};
use crate::{
    bisq::payload::NodeAddress,
    prelude::{
        net::{TcpListener, TcpStream},
        *,
    },
};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

pub struct TorConfig {
    pub hidden_service_port: u16,
    pub tc_port: u16,
    pub private_key_path: PathBuf,
}

pub struct Server<D: SendableDispatcher> {
    listen_port: u16,
    tor_conf: Option<TorConfig>,
    peers: Addr<Peers<D>>,
    bootstrap: Addr<Bootstrap<D>>,
}
pub fn start<D: SendableDispatcher>(
    listen_port: u16,
    peers: Addr<Peers<D>>,
    bootstrap: Addr<Bootstrap<D>>,
    tor_conf: Option<TorConfig>,
) -> Addr<Server<D>> {
    Server {
        listen_port,
        tor_conf,
        peers,
        bootstrap,
    }
    .start()
}
impl<D: SendableDispatcher> Actor for Server<D> {
    type Context = Context<Server<D>>;
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
impl<D: SendableDispatcher> StreamHandler<TcpStream, io::Error> for Server<D> {
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
