use super::convert;
use crate::{
    bisq::{
        payload::{kind::*, *},
        PersistentMessageHash, SequencedMessageHash,
    },
    domain::{
        offer::{message::*, OfferBook},
        statistics::{StatsCache, Trade},
        CommandResult,
    },
    p2p::{dispatch::Receive, message::Broadcast, Broadcaster, ConnectionId},
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    mem,
    time::SystemTime,
};

pub struct DataRouter {
    offer_book: Addr<OfferBook>,
    broadcaster: Addr<Broadcaster>,
    #[cfg(feature = "statistics")]
    stats_cache: StatsCache,
    sequenced_message_info: HashMap<SequencedMessageHash, SequencedMessageInfo>,
    persistent_message_info: HashSet<PersistentMessageHash>,
}
impl Actor for DataRouter {
    type Context = Context<Self>;
}
struct SequencedMessageInfo {
    last_delivery: SystemTime,
    sequence: i32,
    owner_pub_key: Vec<u8>,
    original_payload: StoragePayload,
}
trait ResultHandler: FnOnce(Result<CommandResult, MailboxError>) -> Result<(), ()> {}
impl<F> ResultHandler for F where F: FnOnce(Result<CommandResult, MailboxError>) -> Result<(), ()> {}

impl DataRouter {
    #[allow(unused_variables)]
    pub fn start(
        offer_book: Addr<OfferBook>,
        broadcaster: Addr<Broadcaster>,
        stats_cache: Option<StatsCache>,
    ) -> Addr<DataRouter> {
        DataRouter {
            offer_book,
            broadcaster,
            #[cfg(feature = "statistics")]
            stats_cache: stats_cache.expect("StatsCache missing"),
            sequenced_message_info: HashMap::new(),
            persistent_message_info: HashSet::new(),
        }
        .start()
    }
    fn ignore_command_result() -> impl ResultHandler {
        |_result| Ok(())
    }
    fn handle_command_result<M>(&self, origin: ConnectionId, original: M) -> impl ResultHandler
    where
        M: Into<network_envelope::Message> + Send + Clone + 'static,
    {
        let broadcaster = self.broadcaster.clone();
        move |result| {
            if let Ok(CommandResult::Accepted) = result {
                arbiter_spawn!(broadcaster.send(Broadcast(original, Some(origin))));
            }
            Ok(())
        }
    }

    fn route_bootstrap_data(
        &mut self,
        data: Vec<StorageEntryWrapper>,
        payloads: Vec<PersistableNetworkPayload>,
    ) {
        data.into_iter().for_each(|w| {
            self.route_storage_entry_wrapper(Some(w), Self::ignore_command_result());
        });
        let mut trades = if cfg!(feature = "statistics") {
            Some(Vec::new())
        } else {
            None
        };
        payloads.into_iter().for_each(|p| {
            self.route_persistable_network_payload(
                Some(p),
                trades.as_mut(),
                Self::ignore_command_result(),
            );
        });
        #[cfg(feature = "statistics")]
        arbiter_spawn!(self.stats_cache.bootstrap(trades.unwrap()));
    }
    fn should_deliver_sequenced(
        &mut self,
        hash: SequencedMessageHash,
        sequence: i32,
        owner_pub_key: Vec<u8>,
        original_payload: &StoragePayload,
    ) -> bool {
        match self.sequenced_message_info.get_mut(&hash) {
            Some(ref mut info) if sequence > info.sequence => {
                info.sequence = sequence;
                info.last_delivery = SystemTime::now();
                true
            }
            None => {
                self.sequenced_message_info.insert(
                    hash,
                    SequencedMessageInfo {
                        sequence,
                        last_delivery: SystemTime::now(),
                        owner_pub_key,
                        original_payload: original_payload.clone(),
                    },
                );
                true
            }
            _ => false,
        }
    }
    fn route_storage_entry_wrapper(
        &mut self,
        entry_wrapper: Option<StorageEntryWrapper>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        match entry_wrapper?.message? {
            storage_entry_wrapper::Message::ProtectedStorageEntry(entry) => {
                self.route_protected_storage_entry(false, Some(entry), result_handler);
            }
            storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(entry) => {
                self.route_protected_storage_entry(false, entry.entry, result_handler);
            }
        }
        Some(())
    }
    fn route_protected_storage_entry(
        &mut self,
        remove_data: bool,
        entry: Option<ProtectedStorageEntry>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        let mut entry = entry?;
        let bisq_hash = entry.verify()?;
        if !self.should_deliver_sequenced(
            bisq_hash,
            entry.sequence_number,
            mem::replace(&mut entry.owner_pub_key_bytes, Vec::new()),
            entry.storage_payload.as_ref()?,
        ) {
            return None;
        }
        #[allow(clippy::single_match)]
        match (&entry).into() {
            StoragePayloadKind::OfferPayload => {
                convert::open_offer(entry, bisq_hash)
                    .map(|offer| {
                        if remove_data {
                            arbiter_spawn!(self
                                .offer_book
                                .send(RemoveOffer(offer))
                                .then(result_handler))
                        } else {
                            arbiter_spawn!(self
                                .offer_book
                                .send(AddOffer(offer))
                                .then(result_handler))
                        }
                    })
                    .or_else(|| {
                        warn!("Offer didn't convert {:?}", bisq_hash);
                        None
                    });
            }
            _ => (),
        }
        Some(())
    }
    #[allow(unused_variables)]
    fn route_persistable_network_payload(
        &mut self,
        payload: Option<PersistableNetworkPayload>,
        trades: Option<&mut Vec<Trade>>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        let payload = payload?;
        let bisq_hash = payload.bisq_hash();
        if !self.persistent_message_info.insert(bisq_hash) {
            return None;
        }

        #[warn(clippy::single_match)]
        match PersistableNetworkPayloadKind::from(&payload) {
            #[cfg(feature = "statistics")]
            PersistableNetworkPayloadKind::TradeStatistics2 => {
                if let Some(trade) = convert::trade_statistics2(payload) {
                    if let Some(trades) = trades {
                        trades.push(trade)
                    } else {
                        arbiter_spawn!(self.stats_cache.add(trade).then(result_handler))
                    }
                }
            }
            _ => (),
        }
        Some(())
    }
}

pub enum DataRouterDispatch {
    Bootstrap(Vec<StorageEntryWrapper>, Vec<PersistableNetworkPayload>),
    RefreshOffer(RefreshOfferMessage),
    AddData(AddDataMessage),
    RemoveData(RemoveDataMessage),
    AddPersistableNetworkPayload(AddPersistableNetworkPayloadMessage),
}

impl Handler<Receive<DataRouterDispatch>> for DataRouter {
    type Result = ();
    fn handle(
        &mut self,
        Receive(origin, dispatch): Receive<DataRouterDispatch>,
        _ctx: &mut Self::Context,
    ) {
        match dispatch {
            DataRouterDispatch::Bootstrap(data, persistable_network_payloads) => {
                self.route_bootstrap_data(data, persistable_network_payloads)
            }
            DataRouterDispatch::RefreshOffer(msg) => {
                let hash = msg.payload_hash();
                if let Some(ref mut info) = self.sequenced_message_info.get_mut(&hash) {
                    if info.sequence < msg.sequence_number
                        && msg
                            .verify(&*info.owner_pub_key, &info.original_payload)
                            .is_some()
                    {
                        info.sequence = msg.sequence_number;
                        info.last_delivery = SystemTime::now();
                        Arbiter::spawn(
                            self.offer_book
                                .send(convert::refresh_offer(&msg))
                                .then(self.handle_command_result(origin, msg)),
                        );
                    }
                }
            }
            DataRouterDispatch::AddData(data) => {
                self.route_storage_entry_wrapper(
                    data.entry.clone(),
                    self.handle_command_result(origin, data),
                );
            }
            DataRouterDispatch::RemoveData(data) => {
                self.route_protected_storage_entry(
                    true,
                    data.protected_storage_entry.clone(),
                    self.handle_command_result(origin, data),
                );
            }
            DataRouterDispatch::AddPersistableNetworkPayload(msg) => {
                self.route_persistable_network_payload(
                    msg.payload.as_ref().map(Clone::clone),
                    None,
                    self.handle_command_result(origin, msg),
                );
            }
        }
    }
}

impl PayloadExtractor for DataRouterDispatch {
    type Extraction = DataRouterDispatch;
    fn extract(msg: network_envelope::Message) -> Extract<Self::Extraction> {
        match msg {
            network_envelope::Message::GetDataResponse(GetDataResponse {
                data_set,
                persistable_network_payload_items,
                ..
            }) => Extract::Succeeded(DataRouterDispatch::Bootstrap(
                data_set,
                persistable_network_payload_items,
            )),
            network_envelope::Message::AddDataMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::AddData(msg))
            }
            network_envelope::Message::RemoveDataMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::RemoveData(msg))
            }
            network_envelope::Message::RefreshOfferMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::RefreshOffer(msg))
            }
            network_envelope::Message::AddPersistableNetworkPayloadMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::AddPersistableNetworkPayload(msg))
            }
            _ => Extract::Failed(msg),
        }
    }
}
