use super::convert;
use crate::{
    bisq::payload::{kind::*, *},
    domain::{
        offer::{message::*, OfferBook},
        stats::StatsCache,
        CommandResult,
    },
    p2p::{dispatch::Receive, message::Broadcast, Broadcaster, ConnectionId},
    prelude::*,
};

pub struct DataRouter {
    offer_book: Addr<OfferBook>,
    broadcaster: Addr<Broadcaster>,
    #[cfg(feature = "stats")]
    stats_cache: StatsCache,
}
impl Actor for DataRouter {
    type Context = Context<Self>;
}
trait ResultHandler: FnOnce(Result<CommandResult, MailboxError>) -> Result<(), ()> {}
impl<F> ResultHandler for F where F: FnOnce(Result<CommandResult, MailboxError>) -> Result<(), ()> {}

impl DataRouter {
    pub fn start(
        offer_book: Addr<OfferBook>,
        broadcaster: Addr<Broadcaster>,
        stats_cache: Option<StatsCache>,
    ) -> Addr<DataRouter> {
        DataRouter {
            offer_book,
            broadcaster,
            #[cfg(feature = "stats")]
            stats_cache: stats_cache.expect("StatsCache missing"),
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
                Arbiter::spawn(
                    broadcaster
                        .send(Broadcast(original, Some(origin)))
                        .then(|_| Ok(())),
                );
            }
            Ok(())
        }
    }

    fn route_bootstrap_data(&self, data: Vec<StorageEntryWrapper>) {
        data.into_iter().for_each(|w| {
            self.route_storage_entry_wrapper(Some(w), Self::ignore_command_result());
        })
    }
    fn route_storage_entry_wrapper(
        &self,
        entry_wrapper: Option<StorageEntryWrapper>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        match entry_wrapper?.message? {
            storage_entry_wrapper::Message::ProtectedStorageEntry(entry) => {
                self.route_protected_storage_entry(Some(entry), result_handler);
            }
            storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(entry) => {
                self.route_protected_storage_entry(entry.entry, result_handler);
            }
        }
        .into()
    }
    fn route_protected_storage_entry(
        &self,
        entry: Option<ProtectedStorageEntry>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        let entry = entry?;
        match (&entry).into() {
            StoragePayloadKind::OfferPayload => Arbiter::spawn(
                self.offer_book
                    .send(AddOffer(convert::open_offer(entry).unwrap()))
                    .then(result_handler),
            ),
            _ => (),
        }
        .into()
    }
    fn route_persistable_network_payload(
        &self,
        payload: Option<PersistableNetworkPayload>,
        result_handler: impl ResultHandler + 'static,
    ) -> Option<()> {
        let payload = payload?;
        match (&payload).into() {
            #[cfg(feature = "stats")]
            PersistableNetworkPayloadKind::TradeStatistics2 => Arbiter::spawn(
                self.stats_cache
                    .add(convert::trade_statistics2(payload).unwrap())
                    .then(result_handler),
            ),
            _ => (),
        }
        .into()
    }
}

pub enum DataRouterDispatch {
    Bootstrap(Vec<StorageEntryWrapper>, Vec<PersistableNetworkPayload>),
    RefreshOffer(RefreshOfferMessage),
    AddData(AddDataMessage),
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
            DataRouterDispatch::Bootstrap(data, _) => self.route_bootstrap_data(data),
            DataRouterDispatch::RefreshOffer(msg) => Arbiter::spawn(
                self.offer_book
                    .send(convert::refresh_offer(&msg))
                    .then(self.handle_command_result(origin, msg)),
            ),
            DataRouterDispatch::AddData(data) => {
                self.route_storage_entry_wrapper(
                    data.entry.clone(),
                    self.handle_command_result(origin, data),
                );
            }
            DataRouterDispatch::AddPersistableNetworkPayload(msg) => {
                self.route_persistable_network_payload(
                    msg.payload.as_ref().map(Clone::clone),
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
