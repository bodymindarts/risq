use crate::{bisq::NodeAddress, p2p::ConnectionId};
use std::{collections::HashMap, sync::Arc, time::SystemTime};

#[derive(Clone)]
pub struct ConnectionStatus {
    pub addr: Option<NodeAddress>,
    pub alive_at: SystemTime,
}

#[derive(Clone)]
pub struct Status {
    connections: Arc<HashMap<ConnectionId, ConnectionStatus>>,
}

impl Status {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(HashMap::new()),
        }
    }

    pub fn connections(&self) -> impl Iterator<Item = (&ConnectionId, &ConnectionStatus)> {
        self.connections.iter()
    }
    pub fn connection_added(&mut self, id: ConnectionId, addr: Option<NodeAddress>) {
        let connections = Arc::make_mut(&mut self.connections);
        connections.insert(
            id,
            ConnectionStatus {
                addr,
                alive_at: SystemTime::now(),
            },
        );
    }

    pub fn connection_removed(&mut self, id: &ConnectionId) {
        let connections = Arc::make_mut(&mut self.connections);
        connections.remove(id);
    }

    pub fn connection_identified(&mut self, id: &ConnectionId, addr: &NodeAddress) {
        let connections = Arc::make_mut(&mut self.connections);
        let status = connections.get_mut(id).expect("Connection not in status");
        if status.addr.is_none() {
            status.addr = Some(addr.to_owned());
        }
        status.alive_at = SystemTime::now();
    }

    pub fn connection_alive(&mut self, id: &ConnectionId, at: SystemTime) {
        let connections = Arc::make_mut(&mut self.connections);
        connections
            .get_mut(id)
            .expect("Connection not in status")
            .alive_at = at;
    }
}
