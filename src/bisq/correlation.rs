use super::payload::{network_envelope::Message, *};

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CorrelationId {
    S(String),
    I(i32),
}
pub trait Correlated {
    fn correlation_id(&self) -> Option<CorrelationId>;
}

pub trait ResponseExtractor {
    type Response: Send;
    fn extract(msg: Message) -> Self::Response;
}
impl ResponseExtractor for PreliminaryGetDataRequest {
    type Response = GetDataResponse;

    fn extract(msg: Message) -> Self::Response {
        match msg {
            Message::GetDataResponse(request) => Some(request),
            _ => None,
        }
        .expect("Msg was not the extpected response type")
    }
}
impl ResponseExtractor for GetUpdatedDataRequest {
    type Response = GetDataResponse;

    fn extract(msg: Message) -> Self::Response {
        match msg {
            Message::GetDataResponse(request) => Some(request),
            _ => None,
        }
        .expect("Msg was not the extpected response type")
    }
}

impl Correlated for Message {
    fn correlation_id(&self) -> Option<CorrelationId> {
        match self {
            Message::PreliminaryGetDataRequest(request) => Some(CorrelationId::I(request.nonce)),
            Message::GetDataResponse(response) => Some(CorrelationId::I(response.request_nonce)),
            Message::GetUpdatedDataRequest(request) => Some(CorrelationId::I(request.nonce)),
            Message::GetPeersRequest(request) => Some(CorrelationId::I(request.nonce)),
            Message::GetPeersResponse(response) => Some(CorrelationId::I(response.request_nonce)),
            Message::Ping(request) => Some(CorrelationId::I(request.nonce)),
            Message::Pong(response) => Some(CorrelationId::I(response.request_nonce)),
            Message::OfferAvailabilityRequest(request) => {
                Some(CorrelationId::S(request.offer_id.to_owned()))
            }
            Message::OfferAvailabilityResponse(response) => {
                Some(CorrelationId::S(response.offer_id.to_owned()))
            }
            Message::RefreshOfferMessage(_) => None,
            Message::AddDataMessage(_) => None,
            Message::RemoveDataMessage(_) => None,
            Message::RemoveMailboxDataMessage(_) => None,
            Message::CloseConnectionMessage(_) => None,
            Message::PrefixedSealedAndSignedMessage(_) => None,
            Message::PayDepositRequest(_) => None,
            Message::PublishDepositTxRequest(_) => None,
            Message::DepositTxPublishedMessage(_) => None,
            Message::CounterCurrencyTransferStartedMessage(_) => None,
            Message::PayoutTxPublishedMessage(_) => None,
            Message::OpenNewDisputeMessage(_) => None,
            Message::PeerOpenedDisputeMessage(_) => None,
            Message::ChatMessage(_) => None,
            Message::DisputeResultMessage(_) => None,
            Message::PeerPublishedDisputePayoutTxMessage(_) => None,
            Message::PrivateNotificationMessage(_) => None,
            Message::GetBlocksRequest(request) => Some(CorrelationId::I(request.nonce)),
            Message::GetBlocksResponse(response) => Some(CorrelationId::I(response.request_nonce)),
            Message::NewBlockBroadcastMessage(_) => None,
            Message::AddPersistableNetworkPayloadMessage(_) => None,
            Message::AckMessage(_) => None,
            Message::RepublishGovernanceDataRequest(_) => None,
            Message::NewDaoStateHashMessage(_) => None,
            Message::GetDaoStateHashesRequest(request) => Some(CorrelationId::I(request.nonce)),
            Message::GetDaoStateHashesResponse(response) => {
                Some(CorrelationId::I(response.request_nonce))
            }
            Message::NewProposalStateHashMessage(_) => None,
            Message::GetProposalStateHashesRequest(request) => {
                Some(CorrelationId::I(request.nonce))
            }
            Message::GetProposalStateHashesResponse(response) => {
                Some(CorrelationId::I(response.request_nonce))
            }
            Message::NewBlindVoteStateHashMessage(_) => None,
            Message::GetBlindVoteStateHashesRequest(request) => {
                Some(CorrelationId::I(request.nonce))
            }
            Message::GetBlindVoteStateHashesResponse(response) => {
                Some(CorrelationId::I(response.request_nonce))
            }
            Message::BundleOfEnvelopes(_) => None,
            Message::MediatedPayoutTxSignatureMessage(_) => None,
            Message::MediatedPayoutTxPublishedMessage(_) => None,
        }
    }
}
