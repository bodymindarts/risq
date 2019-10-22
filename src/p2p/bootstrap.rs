use super::{
    connection::{Connection, ConnectionId, Request},
    dispatch::SendableDispatcher,
    peers::{Peers, SeedConnection},
    server::event::ServerStarted,
};
use crate::{
    bisq::{
        constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
        payload::*,
    },
    error::Error,
    prelude::{sync::oneshot, *},
};
use rand::{seq::SliceRandom, thread_rng};

pub struct Bootstrap<D: SendableDispatcher> {
    network: BaseCurrencyNetwork,
    proxy_port: Option<u16>,
    addr_notify: Option<oneshot::Sender<NodeAddress>>,
    addr_rec: Option<oneshot::Receiver<NodeAddress>>,
    seed_nodes: Vec<NodeAddress>,
    peers: Addr<Peers<D>>,
    dispatcher: D,
}
impl<D: SendableDispatcher> Actor for Bootstrap<D> {
    type Context = Context<Bootstrap<D>>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = self.seed_nodes.pop().expect("No seed nodes defined");
        ctx.spawn(
            fut::wrap_future(bootstrap_from_seed(
                addr.clone(),
                self.addr_rec.take().expect("Receiver already removed"),
                self.network,
                self.dispatcher.clone(),
                self.proxy_port,
            ))
            .map_err(|_, _, _| ())
            .and_then(move |seed_result, bootstrap: &mut Bootstrap<D>, _ctx| {
                fut::wrap_future(
                    bootstrap
                        .peers
                        .send(SeedConnection(
                            addr,
                            seed_result.connection_id,
                            seed_result.connection,
                        ))
                        .map_err(|_| ()),
                )
            })
            .map(|_, _, ctx| ctx.stop()),
        );
    }
}
impl<D: SendableDispatcher> Handler<ServerStarted> for Bootstrap<D> {
    type Result = ();
    fn handle(&mut self, ServerStarted(local_addr): ServerStarted, _ctx: &mut Self::Context) {
        self.addr_notify
            .take()
            .expect("Local addr notifier already used")
            .send(local_addr)
            .map_err(|e| error!("ERR: {:?}", e))
            .expect("Couldn't send local address");
    }
}
impl<D: SendableDispatcher> Bootstrap<D> {
    pub fn start(
        network: BaseCurrencyNetwork,
        peers: Addr<Peers<D>>,
        dispatcher: D,
        proxy_port: Option<u16>,
    ) -> Addr<Bootstrap<D>> {
        let mut seed_nodes = seed_nodes(&network);
        seed_nodes.shuffle(&mut thread_rng());
        let (addr_notify, addr_rec) = oneshot::channel();
        Self {
            network,
            addr_notify: Some(addr_notify),
            addr_rec: Some(addr_rec),
            proxy_port,
            seed_nodes,
            peers,
            dispatcher,
        }
        .start()
    }
}
struct SeedResult {
    connection: Addr<Connection>,
    connection_id: ConnectionId,
}
fn bootstrap_from_seed<D: SendableDispatcher>(
    seed_addr: NodeAddress,
    local_addr: oneshot::Receiver<NodeAddress>,
    network: BaseCurrencyNetwork,
    dispatcher: D,
    proxy_port: Option<u16>,
) -> impl Future<Item = SeedResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: gen_nonce(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    info!("Bootstrapping from seed: {:?}", seed_addr);
    Connection::open(seed_addr, network.into(), dispatcher.clone(), proxy_port)
        .and_then(|(id, conn)| {
            debug!("Sending PreliminaryGetDataRequest to seed.");
            conn.send(Request(preliminary_get_data_request))
                .flatten()
                .map(move |response| (id, conn, response))
        })
        .and_then(move |(id, conn, preliminary_data_response)| {
            debug!(
                "Preliminary data response has {} items",
                preliminary_data_response.data_set.len()
                    + preliminary_data_response
                        .persistable_network_payload_items
                        .len()
            );
            let excluded_keys = get_excluded_keys(&preliminary_data_response);
            dispatcher.dispatch(id, preliminary_data_response.into());

            local_addr
                .map(move |addr| {
                    (
                        GetUpdatedDataRequest {
                            sender_node_address: addr.into(),
                            nonce: gen_nonce(),
                            excluded_keys,
                        },
                        id,
                        conn,
                        dispatcher,
                    )
                })
                .map_err(|e| e.into())
        })
        .and_then(|(request, id, conn, dispatcher)| {
            debug!("Sending GetUpdatedDataRequest to seed.");
            conn.send(Request(request))
                .flatten()
                .map(move |get_updated_data_response| {
                    debug!(
                        "Update data response has {} items",
                        get_updated_data_response.data_set.len()
                            + get_updated_data_response
                                .persistable_network_payload_items
                                .len()
                    );
                    dispatcher.dispatch(id, get_updated_data_response.into());
                    SeedResult {
                        connection_id: id,
                        connection: conn,
                    }
                })
        })
}
fn get_excluded_keys(preliminary_data_response: &GetDataResponse) -> Vec<Vec<u8>> {
    preliminary_data_response
        .data_set
        .iter()
        .map(|w| w.message.as_ref().expect("Couldn't unwrap message"))
        .map(|m| match m {
            storage_entry_wrapper::Message::ProtectedStorageEntry(entry) => entry,
            storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(mailbox_entry) => {
                mailbox_entry
                    .entry
                    .as_ref()
                    .expect("Couldn't unwrap StorageEntry")
            }
        })
        .map(|entry| {
            entry
                .storage_payload
                .as_ref()
                .expect("Couldn't unwrap storage_payload")
                .bisq_hash()
                .into()
        })
        .chain(
            preliminary_data_response
                .persistable_network_payload_items
                .iter()
                .map(PersistableNetworkPayload::bisq_hash)
                .map(Vec::<u8>::from),
        )
        .collect()
}
