use crate::bisq::payload::*;
use crate::dispatch::Receive;
use actix::{Actor, Addr, Context, Handler, Message};

pub struct SharedData {}
impl Actor for SharedData {
    type Context = Context<Self>;
}
impl SharedData {
    pub fn start() -> Addr<SharedData> {
        SharedData {}.start()
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
        match entry
            .storage_payload
            .expect("Couldn't unwrap ProtectedStorageEntry.storage_payload")
            .message
            .expect("Couldn't unwrap StoragePayload.message")
        {
            storage_payload::Message::OfferPayload(offer_payload) => {
                debug!("Received offer payload {:?}", offer_payload)
            }
            _ => (),
        }
    }
}

impl Handler<Receive<SharedDataDispatch>> for SharedData {
    type Result = ();
    fn handle(
        &mut self,
        Receive(_, dispatch): Receive<SharedDataDispatch>,
        _ctx: &mut Self::Context,
    ) {
        match dispatch {
            SharedDataDispatch::Bootstrap(data, _) => self.distribute_bootstrap_data(data),
        }
    }
}

pub enum SharedDataDispatch {
    Bootstrap(Vec<StorageEntryWrapper>, Vec<PersistableNetworkPayload>),
}
impl PayloadExtractor for SharedDataDispatch {
    type Extraction = SharedDataDispatch;
    fn extract(msg: network_envelope::Message) -> Extract<Self::Extraction> {
        match msg {
            network_envelope::Message::GetDataResponse(GetDataResponse {
                data_set,
                persistable_network_payload_items,
                ..
            }) => Extract::Succeeded(SharedDataDispatch::Bootstrap(
                data_set,
                persistable_network_payload_items,
            )),
            _ => Extract::Failed(msg),
        }
    }
}
