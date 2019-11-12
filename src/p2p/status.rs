use super::{bootstrap::BootstrapState, connection::ConnectionId};
use crate::bisq::NodeAddress;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard},
    time::SystemTime,
};

#[derive(Clone)]
pub struct ConnectionStatus {
    pub addr: Option<NodeAddress>,
    pub alive_at: SystemTime,
}

#[derive(Clone)]
pub struct Status {
    bootstrap_state: Arc<RwLock<BootstrapState>>,
    connections: Arc<RwLock<HashMap<ConnectionId, ConnectionStatus>>>,
}

impl Status {
    pub fn new(bootstrap_state: Arc<RwLock<BootstrapState>>) -> Self {
        Self {
            bootstrap_state,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn bootstrap_state(&self) -> BootstrapState {
        *self
            .bootstrap_state
            .read()
            .expect("Corrupted lock in status")
    }

    pub fn connections(&self) -> RwLockReadGuard<HashMap<ConnectionId, ConnectionStatus>> {
        self.connections.read().expect("Corrupted lock in status")
    }

    pub fn connection_added(&mut self, id: ConnectionId, addr: Option<NodeAddress>) {
        self.connections
            .write()
            .expect("Corrupted lock in status")
            .insert(
                id,
                ConnectionStatus {
                    addr,
                    alive_at: SystemTime::now(),
                },
            );
    }

    pub fn connection_removed(&mut self, id: &ConnectionId) {
        self.connections
            .write()
            .expect("Corrupted lock in status")
            .remove(id);
    }

    pub fn connection_identified(&mut self, id: &ConnectionId, addr: &NodeAddress) {
        let mut connections = self.connections.write().expect("Corrupted lock in status");
        let status = connections.get_mut(id).expect("Connection not in status");
        if status.addr.is_none() {
            status.addr = Some(addr.to_owned());
        }
        status.alive_at = SystemTime::now();
    }

    pub fn connection_alive(&mut self, id: &ConnectionId, at: SystemTime) {
        self.connections
            .write()
            .expect("Corrupted lock in status")
            .get_mut(id)
            .expect("Connection not in status")
            .alive_at = at;
    }
}
