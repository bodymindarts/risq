use crate::bisq::payload::{kind::*, *};
use crate::dispatch::Receive;
use crate::domain::{conversion, offer_book::*, OpenOffer};
use actix::{Actor, Addr, Arbiter, Context, Handler};
use std::time::{Duration, SystemTime};
use tokio::prelude::future::Future;

pub struct DataRouter {
    offer_book: Addr<OfferBook>,
}
impl Actor for DataRouter {
    type Context = Context<Self>;
}
impl DataRouter {
    pub fn start() -> Addr<DataRouter> {
        DataRouter {
            offer_book: OfferBook::start(),
        }
        .start()
    }
    pub fn distribute_bootstrap_data(&self, data: Vec<StorageEntryWrapper>) {
        data.into_iter().for_each(|w| {
            match w
                .message
                .expect("Couldn't unwrap StorageEntryWrapper.message")
            {
                storage_entry_wrapper::Message::ProtectedStorageEntry(entry) => {
                    self.distribute_protected_storage_entry(entry);
                }
                storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(entry) => {
                    self.distribute_protected_storage_entry(
                        entry
                            .entry
                            .expect("Couldn't unwrap ProtectedMailboxStorageEntry.entry"),
                    );
                }
            }
        })
    }
    pub fn distribute_protected_storage_entry(&self, entry: ProtectedStorageEntry) -> Option<()> {
        match (&entry).into() {
            StoragePayloadKind::OfferPayload => Arbiter::spawn(
                self.offer_book
                    .send(AddOffer(conversion::open_offer(entry).unwrap()))
                    .then(|_| Ok(())),
            ),
            _ => (),
        }
        .into()
    }
}

pub enum DataRouterDispatch {
    Bootstrap(Vec<StorageEntryWrapper>, Vec<PersistableNetworkPayload>),
    RefreshOffer(RefreshOfferMessage),
}

impl Handler<Receive<DataRouterDispatch>> for DataRouter {
    type Result = ();
    fn handle(
        &mut self,
        Receive(_, dispatch): Receive<DataRouterDispatch>,
        _ctx: &mut Self::Context,
    ) {
        match dispatch {
            DataRouterDispatch::Bootstrap(data, _) => self.distribute_bootstrap_data(data),
            DataRouterDispatch::RefreshOffer(msg) => Arbiter::spawn(
                self.offer_book
                    .send(conversion::refresh_offer(msg))
                    .then(|_| Ok(())),
            ),
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
            network_envelope::Message::RefreshOfferMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::RefreshOffer(msg))
            }
            _ => Extract::Failed(msg),
        }
    }
}
