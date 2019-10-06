use super::convert;
use crate::{
    bisq::payload::{kind::*, *},
    domain::offer_book::*,
    p2p::dispatch::Receive,
};
use actix::{Actor, Addr, Arbiter, Context, Handler};
use tokio::prelude::future::Future;

pub struct DataRouter {
    offer_book: Addr<OfferBook>,
}
impl Actor for DataRouter {
    type Context = Context<Self>;
}
impl DataRouter {
    pub fn start(offer_book: Addr<OfferBook>) -> Addr<DataRouter> {
        DataRouter { offer_book }.start()
    }
    pub fn route_bootstrap_data(&self, data: Vec<StorageEntryWrapper>) {
        data.into_iter().for_each(|w| {
            self.route_storage_entry_wrapper(Some(w));
        })
    }
    pub fn route_storage_entry_wrapper(
        &self,
        entry_wrapper: Option<StorageEntryWrapper>,
    ) -> Option<()> {
        match entry_wrapper?.message? {
            storage_entry_wrapper::Message::ProtectedStorageEntry(entry) => {
                self.route_protected_storage_entry(Some(entry));
            }
            storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(entry) => {
                self.route_protected_storage_entry(entry.entry);
            }
        }
        .into()
    }
    pub fn route_protected_storage_entry(
        &self,
        entry: Option<ProtectedStorageEntry>,
    ) -> Option<()> {
        let entry = entry?;
        match (&entry).into() {
            StoragePayloadKind::OfferPayload => Arbiter::spawn(
                self.offer_book
                    .send(AddOffer(convert::open_offer(entry).unwrap()))
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
    AddData(AddDataMessage),
}

impl Handler<Receive<DataRouterDispatch>> for DataRouter {
    type Result = ();
    fn handle(
        &mut self,
        Receive(_, dispatch): Receive<DataRouterDispatch>,
        _ctx: &mut Self::Context,
    ) {
        match dispatch {
            DataRouterDispatch::Bootstrap(data, _) => self.route_bootstrap_data(data),
            DataRouterDispatch::RefreshOffer(msg) => Arbiter::spawn(
                self.offer_book
                    .send(convert::refresh_offer(msg))
                    .then(|_| Ok(())),
            ),
            DataRouterDispatch::AddData(data) => {
                self.route_storage_entry_wrapper(data.entry);
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
            network_envelope::Message::RefreshOfferMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::RefreshOffer(msg))
            }
            network_envelope::Message::AddDataMessage(msg) => {
                Extract::Succeeded(DataRouterDispatch::AddData(msg))
            }
            _ => Extract::Failed(msg),
        }
    }
}
