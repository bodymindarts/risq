use crate::bisq::{payload::*, BisqHash};
use crate::dispatch::Receive;
use crate::domain::{offer_book::*, OpenOffer};
use actix::{Actor, Addr, Arbiter, Context, Handler};
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
                    self.distribute_protected_storage_entry(entry)
                }
                storage_entry_wrapper::Message::ProtectedMailboxStorageEntry(entry) => self
                    .distribute_protected_storage_entry(
                        entry
                            .entry
                            .expect("Couldn't unwrap ProtectedMailboxStorageEntry.entry"),
                    ),
            }
        })
    }
    pub fn distribute_protected_storage_entry(&self, entry: ProtectedStorageEntry) {
        let storage_payload = entry
            .storage_payload
            .expect("Couldn't unwrap ProtectedStorageEntry.storage_payload");
        let hash: BisqHash = (&storage_payload).into();
        match storage_payload
            .message
            .expect("Couldn't unwrap StoragePayload.message")
        {
            storage_payload::Message::OfferPayload(payload) => Arbiter::spawn(
                self.offer_book
                    .send(AddOffer(OpenOffer::new(hash, payload)))
                    .then(|_| Ok(())),
            ),
            _ => (),
        }
    }
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
        }
    }
}

pub enum DataRouterDispatch {
    Bootstrap(Vec<StorageEntryWrapper>, Vec<PersistableNetworkPayload>),
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
            _ => Extract::Failed(msg),
        }
    }
}
