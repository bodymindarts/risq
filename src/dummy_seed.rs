use crate::{
    bisq::{
        constants::{BaseCurrencyNetwork, LOCAL_CAPABILITIES},
        payload::*,
    },
    p2p::{
        dispatch::*, message::Direct, server, BootstrapState, Broadcaster, ConnectionId, Peers,
        Status,
    },
    prelude::*,
};
use std::path::Path;

#[derive(Clone)]
struct SeedDataResponder(Addr<Broadcaster>);
impl Dispatcher for SeedDataResponder {
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        match msg {
            network_envelope::Message::PreliminaryGetDataRequest(request) => {
                arbiter_spawn!(self.0.send(Direct(
                    GetDataResponse {
                        request_nonce: request.nonce,
                        is_get_updated_data_response: false,
                        data_set: Vec::new(),
                        supported_capabilities: LOCAL_CAPABILITIES.clone(),
                        persistable_network_payload_items: Vec::new(),
                    },
                    conn
                )));
                Dispatch::Consumed
            }
            network_envelope::Message::GetUpdatedDataRequest(request) => {
                arbiter_spawn!(self.0.send(Direct(
                    GetDataResponse {
                        request_nonce: request.nonce,
                        is_get_updated_data_response: true,
                        data_set: Vec::new(),
                        supported_capabilities: LOCAL_CAPABILITIES.clone(),
                        persistable_network_payload_items: Vec::new(),
                    },
                    conn
                )));
                Dispatch::Consumed
            }
            _ => Dispatch::Retained(msg),
        }
    }
}

pub fn run(server_port: u16, _fixtures: Option<&Path>) {
    let sys = System::new("risq");

    let network = BaseCurrencyNetwork::BtcRegtest;
    let broadcaster = Broadcaster::start();
    let bootstrap_state = BootstrapState::init();
    let p2p_status = Status::new(bootstrap_state);

    let peers = Peers::start(
        network,
        broadcaster.clone(),
        p2p_status,
        SeedDataResponder(broadcaster),
        None,
    );

    server::start(server_port, peers, None, None);

    let _ = sys.run();
}
