//
// Protobuffer v3 definitions of network messages and persisted objects.
//

// option java_package = "protobuf";
// option java_multiple_files = true;
///////////////////////////////////////////////////////////////////////////////////////////
// Network messages
///////////////////////////////////////////////////////////////////////////////////////////

/// Those are messages sent over wire
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkEnvelope {
    #[prost(int32, tag = "1")]
    pub message_version: i32,
    #[prost(
        oneof = "network_envelope::Message",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45"
    )]
    pub message: ::std::option::Option<network_envelope::Message>,
}
pub mod network_envelope {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        PreliminaryGetDataRequest(super::PreliminaryGetDataRequest),
        #[prost(message, tag = "3")]
        GetDataResponse(super::GetDataResponse),
        #[prost(message, tag = "4")]
        GetUpdatedDataRequest(super::GetUpdatedDataRequest),
        #[prost(message, tag = "5")]
        GetPeersRequest(super::GetPeersRequest),
        #[prost(message, tag = "6")]
        GetPeersResponse(super::GetPeersResponse),
        #[prost(message, tag = "7")]
        Ping(super::Ping),
        #[prost(message, tag = "8")]
        Pong(super::Pong),
        #[prost(message, tag = "9")]
        OfferAvailabilityRequest(super::OfferAvailabilityRequest),
        #[prost(message, tag = "10")]
        OfferAvailabilityResponse(super::OfferAvailabilityResponse),
        #[prost(message, tag = "11")]
        RefreshOfferMessage(super::RefreshOfferMessage),
        #[prost(message, tag = "12")]
        AddDataMessage(super::AddDataMessage),
        #[prost(message, tag = "13")]
        RemoveDataMessage(super::RemoveDataMessage),
        #[prost(message, tag = "14")]
        RemoveMailboxDataMessage(super::RemoveMailboxDataMessage),
        #[prost(message, tag = "15")]
        CloseConnectionMessage(super::CloseConnectionMessage),
        #[prost(message, tag = "16")]
        PrefixedSealedAndSignedMessage(super::PrefixedSealedAndSignedMessage),
        #[prost(message, tag = "17")]
        PayDepositRequest(super::PayDepositRequest),
        #[prost(message, tag = "18")]
        PublishDepositTxRequest(super::PublishDepositTxRequest),
        #[prost(message, tag = "19")]
        DepositTxPublishedMessage(super::DepositTxPublishedMessage),
        #[prost(message, tag = "20")]
        CounterCurrencyTransferStartedMessage(super::CounterCurrencyTransferStartedMessage),
        #[prost(message, tag = "21")]
        PayoutTxPublishedMessage(super::PayoutTxPublishedMessage),
        #[prost(message, tag = "22")]
        OpenNewDisputeMessage(super::OpenNewDisputeMessage),
        #[prost(message, tag = "23")]
        PeerOpenedDisputeMessage(super::PeerOpenedDisputeMessage),
        #[prost(message, tag = "24")]
        ChatMessage(super::ChatMessage),
        #[prost(message, tag = "25")]
        DisputeResultMessage(super::DisputeResultMessage),
        #[prost(message, tag = "26")]
        PeerPublishedDisputePayoutTxMessage(super::PeerPublishedDisputePayoutTxMessage),
        #[prost(message, tag = "27")]
        PrivateNotificationMessage(super::PrivateNotificationMessage),
        #[prost(message, tag = "28")]
        GetBlocksRequest(super::GetBlocksRequest),
        #[prost(message, tag = "29")]
        GetBlocksResponse(super::GetBlocksResponse),
        #[prost(message, tag = "30")]
        NewBlockBroadcastMessage(super::NewBlockBroadcastMessage),
        #[prost(message, tag = "31")]
        AddPersistableNetworkPayloadMessage(super::AddPersistableNetworkPayloadMessage),
        #[prost(message, tag = "32")]
        AckMessage(super::AckMessage),
        #[prost(message, tag = "33")]
        RepublishGovernanceDataRequest(super::RepublishGovernanceDataRequest),
        #[prost(message, tag = "34")]
        NewDaoStateHashMessage(super::NewDaoStateHashMessage),
        #[prost(message, tag = "35")]
        GetDaoStateHashesRequest(super::GetDaoStateHashesRequest),
        #[prost(message, tag = "36")]
        GetDaoStateHashesResponse(super::GetDaoStateHashesResponse),
        #[prost(message, tag = "37")]
        NewProposalStateHashMessage(super::NewProposalStateHashMessage),
        #[prost(message, tag = "38")]
        GetProposalStateHashesRequest(super::GetProposalStateHashesRequest),
        #[prost(message, tag = "39")]
        GetProposalStateHashesResponse(super::GetProposalStateHashesResponse),
        #[prost(message, tag = "40")]
        NewBlindVoteStateHashMessage(super::NewBlindVoteStateHashMessage),
        #[prost(message, tag = "41")]
        GetBlindVoteStateHashesRequest(super::GetBlindVoteStateHashesRequest),
        #[prost(message, tag = "42")]
        GetBlindVoteStateHashesResponse(super::GetBlindVoteStateHashesResponse),
        #[prost(message, tag = "43")]
        BundleOfEnvelopes(super::BundleOfEnvelopes),
        #[prost(message, tag = "44")]
        MediatedPayoutTxSignatureMessage(super::MediatedPayoutTxSignatureMessage),
        #[prost(message, tag = "45")]
        MediatedPayoutTxPublishedMessage(super::MediatedPayoutTxPublishedMessage),
    }
}
///////////////////////////////////////////////////////////////////////////////////////////
// Implementations of NetworkEnvelope
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundleOfEnvelopes {
    #[prost(message, repeated, tag = "1")]
    pub envelopes: ::std::vec::Vec<NetworkEnvelope>,
}
// get data

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreliminaryGetDataRequest {
    #[prost(int32, tag = "21")]
    pub nonce: i32,
    #[prost(bytes, repeated, tag = "2")]
    pub excluded_keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(int32, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataResponse {
    #[prost(int32, tag = "1")]
    pub request_nonce: i32,
    #[prost(bool, tag = "2")]
    pub is_get_updated_data_response: bool,
    #[prost(message, repeated, tag = "3")]
    pub data_set: ::std::vec::Vec<StorageEntryWrapper>,
    #[prost(int32, repeated, tag = "4")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
    #[prost(message, repeated, tag = "5")]
    pub persistable_network_payload_items: ::std::vec::Vec<PersistableNetworkPayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatedDataRequest {
    #[prost(message, optional, tag = "1")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
    #[prost(bytes, repeated, tag = "3")]
    pub excluded_keys: ::std::vec::Vec<std::vec::Vec<u8>>,
}
// peers

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeersRequest {
    #[prost(message, optional, tag = "1")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
    #[prost(int32, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
    #[prost(message, repeated, tag = "4")]
    pub reported_peers: ::std::vec::Vec<Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeersResponse {
    #[prost(int32, tag = "1")]
    pub request_nonce: i32,
    #[prost(message, repeated, tag = "2")]
    pub reported_peers: ::std::vec::Vec<Peer>,
    #[prost(int32, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    #[prost(int32, tag = "1")]
    pub nonce: i32,
    #[prost(int32, tag = "2")]
    pub last_round_trip_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {
    #[prost(int32, tag = "1")]
    pub request_nonce: i32,
}
// offer

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferAvailabilityRequest {
    #[prost(string, tag = "1")]
    pub offer_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(int64, tag = "3")]
    pub takers_trade_price: i64,
    #[prost(int32, repeated, tag = "4")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
    #[prost(string, tag = "5")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferAvailabilityResponse {
    #[prost(string, tag = "1")]
    pub offer_id: std::string::String,
    #[prost(enumeration = "AvailabilityResult", tag = "2")]
    pub availability_result: i32,
    #[prost(int32, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
    #[prost(string, tag = "4")]
    pub uid: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub arbitrator: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "6")]
    pub mediator: ::std::option::Option<NodeAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshOfferMessage {
    #[prost(bytes, tag = "1")]
    pub hash_of_data_and_seq_nr: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub signature: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub hash_of_payload: std::vec::Vec<u8>,
    #[prost(int32, tag = "4")]
    pub sequence_number: i32,
}
// storage

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataMessage {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<StorageEntryWrapper>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataMessage {
    #[prost(message, optional, tag = "1")]
    pub protected_storage_entry: ::std::option::Option<ProtectedStorageEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMailboxDataMessage {
    #[prost(message, optional, tag = "1")]
    pub protected_storage_entry: ::std::option::Option<ProtectedMailboxStorageEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPersistableNetworkPayloadMessage {
    #[prost(message, optional, tag = "1")]
    pub payload: ::std::option::Option<PersistableNetworkPayload>,
}
// misc

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseConnectionMessage {
    #[prost(string, tag = "1")]
    pub reason: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckMessage {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    /// enum name. e.g.  TradeMessage, DisputeMessage,...
    #[prost(string, tag = "3")]
    pub source_type: std::string::String,
    #[prost(string, tag = "4")]
    pub source_msg_class_name: std::string::String,
    /// uid of source (TradeMessage)
    #[prost(string, tag = "5")]
    pub source_uid: std::string::String,
    /// id of source (tradeId, disputeId)
    #[prost(string, tag = "6")]
    pub source_id: std::string::String,
    /// true if source message was processed successfully
    #[prost(bool, tag = "7")]
    pub success: bool,
    /// optional error message if source message processing failed
    #[prost(string, tag = "8")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrefixedSealedAndSignedMessage {
    #[prost(message, optional, tag = "1")]
    pub node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "2")]
    pub sealed_and_signed: ::std::option::Option<SealedAndSigned>,
    #[prost(bytes, tag = "3")]
    pub address_prefix_hash: std::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub uid: std::string::String,
}
// trade

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayDepositRequest {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(int64, tag = "3")]
    pub trade_amount: i64,
    #[prost(int64, tag = "4")]
    pub trade_price: i64,
    #[prost(int64, tag = "5")]
    pub tx_fee: i64,
    #[prost(int64, tag = "6")]
    pub taker_fee: i64,
    #[prost(bool, tag = "7")]
    pub is_currency_for_taker_fee_btc: bool,
    #[prost(message, repeated, tag = "8")]
    pub raw_transaction_inputs: ::std::vec::Vec<RawTransactionInput>,
    #[prost(int64, tag = "9")]
    pub change_output_value: i64,
    #[prost(string, tag = "10")]
    pub change_output_address: std::string::String,
    #[prost(bytes, tag = "11")]
    pub taker_multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(string, tag = "12")]
    pub taker_payout_address_string: std::string::String,
    #[prost(message, optional, tag = "13")]
    pub taker_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(message, optional, tag = "14")]
    pub taker_payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
    #[prost(string, tag = "15")]
    pub taker_account_id: std::string::String,
    #[prost(string, tag = "16")]
    pub taker_fee_tx_id: std::string::String,
    #[prost(message, repeated, tag = "17")]
    pub accepted_arbitrator_node_addresses: ::std::vec::Vec<NodeAddress>,
    #[prost(message, repeated, tag = "18")]
    pub accepted_mediator_node_addresses: ::std::vec::Vec<NodeAddress>,
    #[prost(message, optional, tag = "19")]
    pub arbitrator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "20")]
    pub mediator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "21")]
    pub uid: std::string::String,
    #[prost(bytes, tag = "22")]
    pub account_age_witness_signature_of_offer_id: std::vec::Vec<u8>,
    #[prost(int64, tag = "23")]
    pub current_date: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishDepositTxRequest {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub maker_payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
    #[prost(string, tag = "3")]
    pub maker_account_id: std::string::String,
    #[prost(string, tag = "4")]
    pub maker_contract_as_json: std::string::String,
    #[prost(string, tag = "5")]
    pub maker_contract_signature: std::string::String,
    #[prost(string, tag = "6")]
    pub maker_payout_address_string: std::string::String,
    #[prost(bytes, tag = "7")]
    pub prepared_deposit_tx: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "8")]
    pub maker_inputs: ::std::vec::Vec<RawTransactionInput>,
    #[prost(bytes, tag = "9")]
    pub maker_multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "10")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "11")]
    pub uid: std::string::String,
    #[prost(bytes, tag = "12")]
    pub account_age_witness_signature_of_prepared_deposit_tx: std::vec::Vec<u8>,
    #[prost(int64, tag = "13")]
    pub current_date: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositTxPublishedMessage {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub deposit_tx: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "4")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterCurrencyTransferStartedMessage {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(string, tag = "2")]
    pub buyer_payout_address: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(bytes, tag = "4")]
    pub buyer_signature: std::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub counter_currency_tx_id: std::string::String,
    #[prost(string, tag = "6")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizePayoutTxRequest {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub seller_signature: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub seller_payout_address: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "5")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayoutTxPublishedMessage {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub payout_tx: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "4")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediatedPayoutTxPublishedMessage {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub payout_tx: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "4")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediatedPayoutTxSignatureMessage {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(bytes, tag = "2")]
    pub tx_signature: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub trade_id: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenNewDisputeMessage {
    #[prost(message, optional, tag = "1")]
    pub dispute: ::std::option::Option<Dispute>,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "3")]
    pub uid: std::string::String,
    #[prost(enumeration = "SupportType", tag = "4")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerOpenedDisputeMessage {
    #[prost(message, optional, tag = "1")]
    pub dispute: ::std::option::Option<Dispute>,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "3")]
    pub uid: std::string::String,
    #[prost(enumeration = "SupportType", tag = "4")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    #[prost(int64, tag = "1")]
    pub date: i64,
    #[prost(string, tag = "2")]
    pub trade_id: std::string::String,
    #[prost(int32, tag = "3")]
    pub trader_id: i32,
    #[prost(bool, tag = "4")]
    pub sender_is_trader: bool,
    #[prost(string, tag = "5")]
    pub message: std::string::String,
    #[prost(message, repeated, tag = "6")]
    pub attachments: ::std::vec::Vec<Attachment>,
    #[prost(bool, tag = "7")]
    pub arrived: bool,
    #[prost(bool, tag = "8")]
    pub stored_in_mailbox: bool,
    #[prost(bool, tag = "9")]
    pub is_system_message: bool,
    #[prost(message, optional, tag = "10")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "11")]
    pub uid: std::string::String,
    #[prost(string, tag = "12")]
    pub send_message_error: std::string::String,
    #[prost(bool, tag = "13")]
    pub acknowledged: bool,
    #[prost(string, tag = "14")]
    pub ack_error: std::string::String,
    #[prost(enumeration = "SupportType", tag = "15")]
    pub r#type: i32,
    #[prost(bool, tag = "16")]
    pub was_displayed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisputeResultMessage {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub dispute_result: ::std::option::Option<DisputeResult>,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(enumeration = "SupportType", tag = "4")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerPublishedDisputePayoutTxMessage {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(bytes, tag = "2")]
    pub transaction: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub trade_id: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(enumeration = "SupportType", tag = "5")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateNotificationMessage {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "3")]
    pub private_notification_payload: ::std::option::Option<PrivateNotificationPayload>,
}
// DAO

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    #[prost(int32, tag = "1")]
    pub from_block_height: i32,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
    #[prost(message, optional, tag = "3")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(int32, repeated, tag = "4")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "1")]
    pub raw_blocks: ::std::vec::Vec<BaseBlock>,
    #[prost(int32, tag = "2")]
    pub request_nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlockBroadcastMessage {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, optional, tag = "1")]
    pub raw_block: ::std::option::Option<BaseBlock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepublishGovernanceDataRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDaoStateHashMessage {
    #[prost(message, optional, tag = "1")]
    pub state_hash: ::std::option::Option<DaoStateHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProposalStateHashMessage {
    #[prost(message, optional, tag = "1")]
    pub state_hash: ::std::option::Option<ProposalStateHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlindVoteStateHashMessage {
    #[prost(message, optional, tag = "1")]
    pub state_hash: ::std::option::Option<BlindVoteStateHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDaoStateHashesRequest {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProposalStateHashesRequest {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlindVoteStateHashesRequest {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(int32, tag = "2")]
    pub nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDaoStateHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub state_hashes: ::std::vec::Vec<DaoStateHash>,
    #[prost(int32, tag = "2")]
    pub request_nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProposalStateHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub state_hashes: ::std::vec::Vec<ProposalStateHash>,
    #[prost(int32, tag = "2")]
    pub request_nonce: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlindVoteStateHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub state_hashes: ::std::vec::Vec<BlindVoteStateHash>,
    #[prost(int32, tag = "2")]
    pub request_nonce: i32,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Payload
///////////////////////////////////////////////////////////////////////////////////////////

// core

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeAddress {
    #[prost(string, tag = "1")]
    pub host_name: std::string::String,
    #[prost(int32, tag = "2")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(message, optional, tag = "1")]
    pub node_address: ::std::option::Option<NodeAddress>,
    #[prost(int64, tag = "2")]
    pub date: i64,
    #[prost(int32, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyRing {
    #[prost(bytes, tag = "1")]
    pub signature_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub encryption_pub_key_bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SealedAndSigned {
    #[prost(bytes, tag = "1")]
    pub encrypted_secret_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub encrypted_payload_with_hmac: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub signature: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub sig_public_key_bytes: std::vec::Vec<u8>,
}
// storage

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePayload {
    #[prost(oneof = "storage_payload::Message", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub message: ::std::option::Option<storage_payload::Message>,
}
pub mod storage_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        Alert(super::Alert),
        #[prost(message, tag = "2")]
        Arbitrator(super::Arbitrator),
        #[prost(message, tag = "3")]
        Mediator(super::Mediator),
        #[prost(message, tag = "4")]
        Filter(super::Filter),
        /// not used anymore from v0.6 on. But leave it for receiving TradeStatistics objects from older
        /// versions and convert it to TradeStatistics2 objects.
        #[prost(message, tag = "5")]
        TradeStatistics(super::TradeStatistics),
        #[prost(message, tag = "6")]
        MailboxStoragePayload(super::MailboxStoragePayload),
        #[prost(message, tag = "7")]
        OfferPayload(super::OfferPayload),
        #[prost(message, tag = "8")]
        TempProposalPayload(super::TempProposalPayload),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistableNetworkPayload {
    #[prost(oneof = "persistable_network_payload::Message", tags = "1, 2, 3, 4, 5")]
    pub message: ::std::option::Option<persistable_network_payload::Message>,
}
pub mod persistable_network_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        AccountAgeWitness(super::AccountAgeWitness),
        #[prost(message, tag = "2")]
        TradeStatistics2(super::TradeStatistics2),
        #[prost(message, tag = "3")]
        ProposalPayload(super::ProposalPayload),
        #[prost(message, tag = "4")]
        BlindVotePayload(super::BlindVotePayload),
        #[prost(message, tag = "5")]
        SignedWitness(super::SignedWitness),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedStorageEntry {
    #[prost(message, optional, tag = "1")]
    pub storage_payload: ::std::option::Option<StoragePayload>,
    #[prost(bytes, tag = "2")]
    pub owner_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(int32, tag = "3")]
    pub sequence_number: i32,
    #[prost(bytes, tag = "4")]
    pub signature: std::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    pub creation_time_stamp: i64,
}
// mailbox

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageEntryWrapper {
    #[prost(oneof = "storage_entry_wrapper::Message", tags = "1, 2")]
    pub message: ::std::option::Option<storage_entry_wrapper::Message>,
}
pub mod storage_entry_wrapper {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        ProtectedStorageEntry(super::ProtectedStorageEntry),
        #[prost(message, tag = "2")]
        ProtectedMailboxStorageEntry(super::ProtectedMailboxStorageEntry),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedMailboxStorageEntry {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<ProtectedStorageEntry>,
    #[prost(bytes, tag = "2")]
    pub receivers_pub_key_bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAndSeqNrPair {
    #[prost(message, optional, tag = "1")]
    pub payload: ::std::option::Option<StoragePayload>,
    #[prost(int32, tag = "2")]
    pub sequence_number: i32,
}
// misc

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateNotificationPayload {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
    #[prost(string, tag = "2")]
    pub signature_as_base64: std::string::String,
    #[prost(bytes, tag = "3")]
    pub sig_public_key_bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccountFilter {
    #[prost(string, tag = "1")]
    pub payment_method_id: std::string::String,
    #[prost(string, tag = "2")]
    pub get_method_name: std::string::String,
    #[prost(string, tag = "3")]
    pub value: std::string::String,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Storage payload
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    #[prost(bool, tag = "3")]
    pub is_update_info: bool,
    #[prost(string, tag = "4")]
    pub signature_as_base64: std::string::String,
    #[prost(bytes, tag = "5")]
    pub owner_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "6")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arbitrator {
    #[prost(message, optional, tag = "1")]
    pub node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, repeated, tag = "2")]
    pub language_codes: ::std::vec::Vec<std::string::String>,
    #[prost(int64, tag = "3")]
    pub registration_date: i64,
    #[prost(string, tag = "4")]
    pub registration_signature: std::string::String,
    #[prost(bytes, tag = "5")]
    pub registration_pub_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(bytes, tag = "7")]
    pub btc_pub_key: std::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub btc_address: std::string::String,
    #[prost(string, tag = "9")]
    pub email_address: std::string::String,
    #[prost(string, tag = "10")]
    pub info: std::string::String,
    #[prost(map = "string, string", tag = "11")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mediator {
    #[prost(message, optional, tag = "1")]
    pub node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, repeated, tag = "2")]
    pub language_codes: ::std::vec::Vec<std::string::String>,
    #[prost(int64, tag = "3")]
    pub registration_date: i64,
    #[prost(string, tag = "4")]
    pub registration_signature: std::string::String,
    #[prost(bytes, tag = "5")]
    pub registration_pub_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(string, tag = "7")]
    pub email_address: std::string::String,
    #[prost(string, tag = "8")]
    pub info: std::string::String,
    #[prost(map = "string, string", tag = "9")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(string, repeated, tag = "1")]
    pub banned_node_address: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub banned_offer_ids: ::std::vec::Vec<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub banned_payment_accounts: ::std::vec::Vec<PaymentAccountFilter>,
    #[prost(string, tag = "4")]
    pub signature_as_base64: std::string::String,
    #[prost(bytes, tag = "5")]
    pub owner_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "6")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub banned_currencies: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub banned_payment_methods: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "9")]
    pub arbitrators: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "10")]
    pub seed_nodes: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "11")]
    pub price_relay_nodes: ::std::vec::Vec<std::string::String>,
    #[prost(bool, tag = "12")]
    pub prevent_public_btc_network: bool,
    #[prost(string, repeated, tag = "13")]
    pub btc_nodes: ::std::vec::Vec<std::string::String>,
    #[prost(bool, tag = "14")]
    pub disable_dao: bool,
    #[prost(string, tag = "15")]
    pub disable_dao_below_version: std::string::String,
    #[prost(string, tag = "16")]
    pub disable_trade_below_version: std::string::String,
    #[prost(string, repeated, tag = "17")]
    pub mediators: ::std::vec::Vec<std::string::String>,
}
/// not used anymore from v0.6 on. But leave it for receiving TradeStatistics objects from older
/// versions and convert it to TradeStatistics2 objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeStatistics {
    #[prost(string, tag = "1")]
    pub base_currency: std::string::String,
    #[prost(string, tag = "2")]
    pub counter_currency: std::string::String,
    #[prost(enumeration = "offer_payload::Direction", tag = "3")]
    pub direction: i32,
    #[prost(int64, tag = "4")]
    pub trade_price: i64,
    #[prost(int64, tag = "5")]
    pub trade_amount: i64,
    #[prost(int64, tag = "6")]
    pub trade_date: i64,
    #[prost(string, tag = "7")]
    pub payment_method_id: std::string::String,
    #[prost(int64, tag = "8")]
    pub offer_date: i64,
    #[prost(bool, tag = "9")]
    pub offer_use_market_based_price: bool,
    #[prost(double, tag = "10")]
    pub offer_market_price_margin: f64,
    #[prost(int64, tag = "11")]
    pub offer_amount: i64,
    #[prost(int64, tag = "12")]
    pub offer_min_amount: i64,
    #[prost(string, tag = "13")]
    pub offer_id: std::string::String,
    #[prost(string, tag = "14")]
    pub deposit_tx_id: std::string::String,
    #[prost(bytes, tag = "15")]
    pub signature_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "16")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeStatistics2 {
    #[prost(string, tag = "1")]
    pub base_currency: std::string::String,
    #[prost(string, tag = "2")]
    pub counter_currency: std::string::String,
    #[prost(enumeration = "offer_payload::Direction", tag = "3")]
    pub direction: i32,
    #[prost(int64, tag = "4")]
    pub trade_price: i64,
    #[prost(int64, tag = "5")]
    pub trade_amount: i64,
    #[prost(int64, tag = "6")]
    pub trade_date: i64,
    #[prost(string, tag = "7")]
    pub payment_method_id: std::string::String,
    #[prost(int64, tag = "8")]
    pub offer_date: i64,
    #[prost(bool, tag = "9")]
    pub offer_use_market_based_price: bool,
    #[prost(double, tag = "10")]
    pub offer_market_price_margin: f64,
    #[prost(int64, tag = "11")]
    pub offer_amount: i64,
    #[prost(int64, tag = "12")]
    pub offer_min_amount: i64,
    #[prost(string, tag = "13")]
    pub offer_id: std::string::String,
    #[prost(string, tag = "14")]
    pub deposit_tx_id: std::string::String,
    #[prost(bytes, tag = "15")]
    pub hash: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "16")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailboxStoragePayload {
    #[prost(message, optional, tag = "1")]
    pub prefixed_sealed_and_signed_message: ::std::option::Option<PrefixedSealedAndSignedMessage>,
    #[prost(bytes, tag = "2")]
    pub sender_pub_key_for_add_operation_bytes: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub owner_pub_key_bytes: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "4")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferPayload {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(int64, tag = "2")]
    pub date: i64,
    #[prost(message, optional, tag = "3")]
    pub owner_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "4")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(enumeration = "offer_payload::Direction", tag = "5")]
    pub direction: i32,
    #[prost(int64, tag = "6")]
    pub price: i64,
    #[prost(double, tag = "7")]
    pub market_price_margin: f64,
    #[prost(bool, tag = "8")]
    pub use_market_based_price: bool,
    #[prost(int64, tag = "9")]
    pub amount: i64,
    #[prost(int64, tag = "10")]
    pub min_amount: i64,
    #[prost(string, tag = "11")]
    pub base_currency_code: std::string::String,
    #[prost(string, tag = "12")]
    pub counter_currency_code: std::string::String,
    /// not used anymore but still required as old clients check for nonNull
    #[prost(message, repeated, tag = "13")]
    pub arbitrator_node_addresses: ::std::vec::Vec<NodeAddress>,
    /// not used anymore but still required as old clients check for nonNull
    #[prost(message, repeated, tag = "14")]
    pub mediator_node_addresses: ::std::vec::Vec<NodeAddress>,
    #[prost(string, tag = "15")]
    pub payment_method_id: std::string::String,
    #[prost(string, tag = "16")]
    pub maker_payment_account_id: std::string::String,
    #[prost(string, tag = "17")]
    pub offer_fee_payment_tx_id: std::string::String,
    #[prost(string, tag = "18")]
    pub country_code: std::string::String,
    #[prost(string, repeated, tag = "19")]
    pub accepted_country_codes: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag = "20")]
    pub bank_id: std::string::String,
    #[prost(string, repeated, tag = "21")]
    pub accepted_bank_ids: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag = "22")]
    pub version_nr: std::string::String,
    #[prost(int64, tag = "23")]
    pub block_height_at_offer_creation: i64,
    #[prost(int64, tag = "24")]
    pub tx_fee: i64,
    #[prost(int64, tag = "25")]
    pub maker_fee: i64,
    #[prost(bool, tag = "26")]
    pub is_currency_for_maker_fee_btc: bool,
    #[prost(int64, tag = "27")]
    pub buyer_security_deposit: i64,
    #[prost(int64, tag = "28")]
    pub seller_security_deposit: i64,
    #[prost(int64, tag = "29")]
    pub max_trade_limit: i64,
    #[prost(int64, tag = "30")]
    pub max_trade_period: i64,
    #[prost(bool, tag = "31")]
    pub use_auto_close: bool,
    #[prost(bool, tag = "32")]
    pub use_re_open_after_auto_close: bool,
    #[prost(int64, tag = "33")]
    pub lower_close_price: i64,
    #[prost(int64, tag = "34")]
    pub upper_close_price: i64,
    #[prost(bool, tag = "35")]
    pub is_private_offer: bool,
    #[prost(string, tag = "36")]
    pub hash_of_challenge: std::string::String,
    #[prost(map = "string, string", tag = "37")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(int32, tag = "38")]
    pub protocol_version: i32,
}
pub mod offer_payload {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        PbError = 0,
        Buy = 1,
        Sell = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAgeWitness {
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub date: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedWitness {
    #[prost(bool, tag = "1")]
    pub signed_by_arbitrator: bool,
    #[prost(bytes, tag = "2")]
    pub witness_hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub signature: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub signer_pub_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "5")]
    pub witness_owner_pub_key: std::vec::Vec<u8>,
    #[prost(int64, tag = "6")]
    pub date: i64,
    #[prost(int64, tag = "7")]
    pub trade_amount: i64,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Dispute payload
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispute {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    #[prost(int32, tag = "3")]
    pub trader_id: i32,
    #[prost(bool, tag = "4")]
    pub dispute_opener_is_buyer: bool,
    #[prost(bool, tag = "5")]
    pub dispute_opener_is_maker: bool,
    #[prost(int64, tag = "6")]
    pub opening_date: i64,
    #[prost(message, optional, tag = "7")]
    pub trader_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(int64, tag = "8")]
    pub trade_date: i64,
    #[prost(message, optional, tag = "9")]
    pub contract: ::std::option::Option<Contract>,
    #[prost(bytes, tag = "10")]
    pub contract_hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "11")]
    pub deposit_tx_serialized: std::vec::Vec<u8>,
    #[prost(bytes, tag = "12")]
    pub payout_tx_serialized: std::vec::Vec<u8>,
    #[prost(string, tag = "13")]
    pub deposit_tx_id: std::string::String,
    #[prost(string, tag = "14")]
    pub payout_tx_id: std::string::String,
    #[prost(string, tag = "15")]
    pub contract_as_json: std::string::String,
    #[prost(string, tag = "16")]
    pub maker_contract_signature: std::string::String,
    #[prost(string, tag = "17")]
    pub taker_contract_signature: std::string::String,
    #[prost(message, optional, tag = "18")]
    pub agent_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(bool, tag = "19")]
    pub is_support_ticket: bool,
    #[prost(message, repeated, tag = "20")]
    pub chat_message: ::std::vec::Vec<ChatMessage>,
    #[prost(bool, tag = "21")]
    pub is_closed: bool,
    #[prost(message, optional, tag = "22")]
    pub dispute_result: ::std::option::Option<DisputeResult>,
    #[prost(string, tag = "23")]
    pub dispute_payout_tx_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    #[prost(string, tag = "1")]
    pub file_name: std::string::String,
    #[prost(bytes, tag = "2")]
    pub bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisputeResult {
    #[prost(string, tag = "1")]
    pub trade_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub trader_id: i32,
    #[prost(enumeration = "dispute_result::Winner", tag = "3")]
    pub winner: i32,
    #[prost(int32, tag = "4")]
    pub reason_ordinal: i32,
    #[prost(bool, tag = "5")]
    pub tamper_proof_evidence: bool,
    #[prost(bool, tag = "6")]
    pub id_verification: bool,
    #[prost(bool, tag = "7")]
    pub screen_cast: bool,
    #[prost(string, tag = "8")]
    pub summary_notes: std::string::String,
    #[prost(message, optional, tag = "9")]
    pub chat_message: ::std::option::Option<ChatMessage>,
    #[prost(bytes, tag = "10")]
    pub arbitrator_signature: std::vec::Vec<u8>,
    #[prost(int64, tag = "11")]
    pub buyer_payout_amount: i64,
    #[prost(int64, tag = "12")]
    pub seller_payout_amount: i64,
    #[prost(bytes, tag = "13")]
    pub arbitrator_pub_key: std::vec::Vec<u8>,
    #[prost(int64, tag = "14")]
    pub close_date: i64,
    #[prost(bool, tag = "15")]
    pub is_loser_publisher: bool,
}
pub mod dispute_result {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Winner {
        PbErrorWinner = 0,
        Buyer = 1,
        Seller = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        PbErrorReason = 0,
        Other = 1,
        Bug = 2,
        Usability = 3,
        Scam = 4,
        ProtocolViolation = 5,
        NoReply = 6,
        BankProblems = 7,
    }
}
///////////////////////////////////////////////////////////////////////////////////////////
// Trade payload
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(message, optional, tag = "1")]
    pub offer_payload: ::std::option::Option<OfferPayload>,
    #[prost(int64, tag = "2")]
    pub trade_amount: i64,
    #[prost(int64, tag = "3")]
    pub trade_price: i64,
    #[prost(string, tag = "4")]
    pub taker_fee_tx_id: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub arbitrator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(bool, tag = "6")]
    pub is_buyer_maker_and_seller_taker: bool,
    #[prost(string, tag = "7")]
    pub maker_account_id: std::string::String,
    #[prost(string, tag = "8")]
    pub taker_account_id: std::string::String,
    #[prost(message, optional, tag = "9")]
    pub maker_payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
    #[prost(message, optional, tag = "10")]
    pub taker_payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
    #[prost(message, optional, tag = "11")]
    pub maker_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(message, optional, tag = "12")]
    pub taker_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(message, optional, tag = "13")]
    pub buyer_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "14")]
    pub seller_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "15")]
    pub maker_payout_address_string: std::string::String,
    #[prost(string, tag = "16")]
    pub taker_payout_address_string: std::string::String,
    #[prost(bytes, tag = "17")]
    pub maker_multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "18")]
    pub taker_multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "19")]
    pub mediator_node_address: ::std::option::Option<NodeAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTransactionInput {
    #[prost(int64, tag = "1")]
    pub index: i64,
    #[prost(bytes, tag = "2")]
    pub parent_transaction: std::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub value: i64,
}
///////////////////////////////////////////////////////////////////////////////////////////
// PaymentAccount payload
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccountPayload {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub payment_method_id: std::string::String,
    /// not used anymore but we need to keep it in PB for backward compatibility
    #[prost(int64, tag = "3")]
    pub max_trade_period: i64,
    #[prost(map = "string, string", tag = "15")]
    pub exclude_from_json_data:
        ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(
        oneof = "payment_account_payload::Message",
        tags = "4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28"
    )]
    pub message: ::std::option::Option<payment_account_payload::Message>,
}
pub mod payment_account_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "4")]
        AliPayAccountPayload(super::AliPayAccountPayload),
        #[prost(message, tag = "5")]
        ChaseQuickPayAccountPayload(super::ChaseQuickPayAccountPayload),
        #[prost(message, tag = "6")]
        ClearXchangeAccountPayload(super::ClearXchangeAccountPayload),
        #[prost(message, tag = "7")]
        CountryBasedPaymentAccountPayload(super::CountryBasedPaymentAccountPayload),
        #[prost(message, tag = "8")]
        CryptoCurrencyAccountPayload(super::CryptoCurrencyAccountPayload),
        #[prost(message, tag = "9")]
        FasterPaymentsAccountPayload(super::FasterPaymentsAccountPayload),
        #[prost(message, tag = "10")]
        InteracETransferAccountPayload(super::InteracETransferAccountPayload),
        #[prost(message, tag = "11")]
        OKPayAccountPayload(super::OkPayAccountPayload),
        #[prost(message, tag = "12")]
        PerfectMoneyAccountPayload(super::PerfectMoneyAccountPayload),
        #[prost(message, tag = "13")]
        SwishAccountPayload(super::SwishAccountPayload),
        #[prost(message, tag = "14")]
        USPostalMoneyOrderAccountPayload(super::UsPostalMoneyOrderAccountPayload),
        #[prost(message, tag = "16")]
        UpholdAccountPayload(super::UpholdAccountPayload),
        #[prost(message, tag = "17")]
        CashAppAccountPayload(super::CashAppAccountPayload),
        #[prost(message, tag = "18")]
        MoneyBeamAccountPayload(super::MoneyBeamAccountPayload),
        #[prost(message, tag = "19")]
        VenmoAccountPayload(super::VenmoAccountPayload),
        #[prost(message, tag = "20")]
        PopmoneyAccountPayload(super::PopmoneyAccountPayload),
        #[prost(message, tag = "21")]
        RevolutAccountPayload(super::RevolutAccountPayload),
        #[prost(message, tag = "22")]
        WeChatPayAccountPayload(super::WeChatPayAccountPayload),
        #[prost(message, tag = "23")]
        MoneyGramAccountPayload(super::MoneyGramAccountPayload),
        #[prost(message, tag = "24")]
        HalCashAccountPayload(super::HalCashAccountPayload),
        #[prost(message, tag = "25")]
        PromptPayAccountPayload(super::PromptPayAccountPayload),
        #[prost(message, tag = "26")]
        AdvancedCashAccountPayload(super::AdvancedCashAccountPayload),
        #[prost(message, tag = "27")]
        InstantCryptoCurrencyAccountPayload(super::InstantCryptoCurrencyAccountPayload),
        #[prost(message, tag = "28")]
        JapanBankAccountPayload(super::JapanBankAccountPayload),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliPayAccountPayload {
    #[prost(string, tag = "1")]
    pub account_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeChatPayAccountPayload {
    #[prost(string, tag = "1")]
    pub account_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChaseQuickPayAccountPayload {
    #[prost(string, tag = "1")]
    pub email: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearXchangeAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub email_or_mobile_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountryBasedPaymentAccountPayload {
    #[prost(string, tag = "1")]
    pub country_code: std::string::String,
    #[prost(
        oneof = "country_based_payment_account_payload::Message",
        tags = "2, 3, 4, 5, 6, 7"
    )]
    pub message: ::std::option::Option<country_based_payment_account_payload::Message>,
}
pub mod country_based_payment_account_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        BankAccountPayload(super::BankAccountPayload),
        #[prost(message, tag = "3")]
        CashDepositAccountPayload(super::CashDepositAccountPayload),
        #[prost(message, tag = "4")]
        SepaAccountPayload(super::SepaAccountPayload),
        #[prost(message, tag = "5")]
        WesternUnionAccountPayload(super::WesternUnionAccountPayload),
        #[prost(message, tag = "6")]
        SepaInstantAccountPayload(super::SepaInstantAccountPayload),
        #[prost(message, tag = "7")]
        F2fAccountPayload(super::F2fAccountPayload),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BankAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub bank_name: std::string::String,
    #[prost(string, tag = "3")]
    pub bank_id: std::string::String,
    #[prost(string, tag = "4")]
    pub branch_id: std::string::String,
    #[prost(string, tag = "5")]
    pub account_nr: std::string::String,
    #[prost(string, tag = "6")]
    pub account_type: std::string::String,
    #[prost(string, tag = "7")]
    pub holder_tax_id: std::string::String,
    #[prost(string, tag = "8")]
    pub email: std::string::String,
    #[prost(string, tag = "12")]
    pub national_account_id: std::string::String,
    #[prost(oneof = "bank_account_payload::Message", tags = "9, 10, 11")]
    pub message: ::std::option::Option<bank_account_payload::Message>,
}
pub mod bank_account_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "9")]
        NationalBankAccountPayload(super::NationalBankAccountPayload),
        #[prost(message, tag = "10")]
        SameBankAccontPayload(super::SameBankAccountPayload),
        #[prost(message, tag = "11")]
        SpecificBanksAccountPayload(super::SpecificBanksAccountPayload),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NationalBankAccountPayload {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SameBankAccountPayload {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JapanBankAccountPayload {
    #[prost(string, tag = "1")]
    pub bank_name: std::string::String,
    #[prost(string, tag = "2")]
    pub bank_code: std::string::String,
    #[prost(string, tag = "3")]
    pub bank_branch_name: std::string::String,
    #[prost(string, tag = "4")]
    pub bank_branch_code: std::string::String,
    #[prost(string, tag = "5")]
    pub bank_account_type: std::string::String,
    #[prost(string, tag = "6")]
    pub bank_account_name: std::string::String,
    #[prost(string, tag = "7")]
    pub bank_account_number: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecificBanksAccountPayload {
    #[prost(string, repeated, tag = "1")]
    pub accepted_banks: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashDepositAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_email: std::string::String,
    #[prost(string, tag = "3")]
    pub bank_name: std::string::String,
    #[prost(string, tag = "4")]
    pub bank_id: std::string::String,
    #[prost(string, tag = "5")]
    pub branch_id: std::string::String,
    #[prost(string, tag = "6")]
    pub account_nr: std::string::String,
    #[prost(string, tag = "7")]
    pub account_type: std::string::String,
    #[prost(string, tag = "8")]
    pub requirements: std::string::String,
    #[prost(string, tag = "9")]
    pub holder_tax_id: std::string::String,
    #[prost(string, tag = "10")]
    pub national_account_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyGramAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub country_code: std::string::String,
    #[prost(string, tag = "3")]
    pub state: std::string::String,
    #[prost(string, tag = "4")]
    pub email: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HalCashAccountPayload {
    #[prost(string, tag = "1")]
    pub mobile_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WesternUnionAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub city: std::string::String,
    #[prost(string, tag = "3")]
    pub state: std::string::String,
    #[prost(string, tag = "4")]
    pub email: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SepaAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub iban: std::string::String,
    #[prost(string, tag = "3")]
    pub bic: std::string::String,
    #[prost(string, tag = "4")]
    pub email: std::string::String,
    #[prost(string, repeated, tag = "5")]
    pub accepted_country_codes: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SepaInstantAccountPayload {
    #[prost(string, tag = "1")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "2")]
    pub iban: std::string::String,
    #[prost(string, tag = "3")]
    pub bic: std::string::String,
    #[prost(string, repeated, tag = "4")]
    pub accepted_country_codes: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoCurrencyAccountPayload {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantCryptoCurrencyAccountPayload {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FasterPaymentsAccountPayload {
    #[prost(string, tag = "1")]
    pub sort_code: std::string::String,
    #[prost(string, tag = "2")]
    pub account_nr: std::string::String,
    #[prost(string, tag = "3")]
    pub email: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteracETransferAccountPayload {
    #[prost(string, tag = "1")]
    pub email: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
    #[prost(string, tag = "3")]
    pub question: std::string::String,
    #[prost(string, tag = "4")]
    pub answer: std::string::String,
}
/// Deprecated, not used anymore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkPayAccountPayload {
    #[prost(string, tag = "1")]
    pub account_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpholdAccountPayload {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
}
/// Deprecated, not used anymore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashAppAccountPayload {
    #[prost(string, tag = "1")]
    pub cash_tag: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyBeamAccountPayload {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
}
/// Deprecated, not used anymore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VenmoAccountPayload {
    #[prost(string, tag = "1")]
    pub venmo_user_name: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopmoneyAccountPayload {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevolutAccountPayload {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerfectMoneyAccountPayload {
    #[prost(string, tag = "1")]
    pub account_nr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwishAccountPayload {
    #[prost(string, tag = "1")]
    pub mobile_nr: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsPostalMoneyOrderAccountPayload {
    #[prost(string, tag = "1")]
    pub postal_address: std::string::String,
    #[prost(string, tag = "2")]
    pub holder_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct F2fAccountPayload {
    #[prost(string, tag = "1")]
    pub contact: std::string::String,
    #[prost(string, tag = "2")]
    pub city: std::string::String,
    #[prost(string, tag = "3")]
    pub extra_info: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromptPayAccountPayload {
    #[prost(string, tag = "1")]
    pub prompt_pay_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvancedCashAccountPayload {
    #[prost(string, tag = "1")]
    pub account_nr: std::string::String,
}
///////////////////////////////////////////////////////////////////////////////////////////
// PersistableEnvelope
///////////////////////////////////////////////////////////////////////////////////////////

/// Those are persisted to disc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistableEnvelope {
    #[prost(
        oneof = "persistable_envelope::Message",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29"
    )]
    pub message: ::std::option::Option<persistable_envelope::Message>,
}
pub mod persistable_envelope {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        SequenceNumberMap(super::SequenceNumberMap),
        #[prost(message, tag = "2")]
        PersistedEntryMap(super::PersistedEntryMap),
        #[prost(message, tag = "3")]
        PeerList(super::PeerList),
        #[prost(message, tag = "4")]
        AddressEntryList(super::AddressEntryList),
        #[prost(message, tag = "5")]
        NavigationPath(super::NavigationPath),
        #[prost(message, tag = "6")]
        TradableList(super::TradableList),
        /// Was used in pre v0.6.0 version. Not used anymore.
        #[prost(message, tag = "7")]
        TradeStatisticsList(super::TradeStatisticsList),
        #[prost(message, tag = "8")]
        ArbitrationDisputeList(super::ArbitrationDisputeList),
        #[prost(message, tag = "9")]
        PreferencesPayload(super::PreferencesPayload),
        #[prost(message, tag = "10")]
        UserPayload(super::UserPayload),
        #[prost(message, tag = "11")]
        PaymentAccountList(super::PaymentAccountList),
        // deprecated
        // BsqState bsq_state = 12; // not used but as other non-dao data have a higher index number we leave it to make clear that we cannot change following indexes
        #[prost(message, tag = "13")]
        AccountAgeWitnessStore(super::AccountAgeWitnessStore),
        #[prost(message, tag = "14")]
        TradeStatistics2Store(super::TradeStatistics2Store),
        /// we need to keep id 15 here otherwise the reading of the old data structure would not work anymore.
        /// can be removed after most people have updated as the reading of the PersistableNetworkPayloadList
        /// is not mandatory.
        #[prost(message, tag = "15")]
        PersistableNetworkPayloadList(super::PersistableNetworkPayloadList),
        #[prost(message, tag = "16")]
        ProposalStore(super::ProposalStore),
        #[prost(message, tag = "17")]
        TempProposalStore(super::TempProposalStore),
        #[prost(message, tag = "18")]
        BlindVoteStore(super::BlindVoteStore),
        #[prost(message, tag = "19")]
        MyProposalList(super::MyProposalList),
        #[prost(message, tag = "20")]
        BallotList(super::BallotList),
        #[prost(message, tag = "21")]
        MyVoteList(super::MyVoteList),
        #[prost(message, tag = "22")]
        MyBlindVoteList(super::MyBlindVoteList),
        #[prost(message, tag = "23")]
        MeritList(super::MeritList),
        #[prost(message, tag = "24")]
        DaoStateStore(super::DaoStateStore),
        #[prost(message, tag = "25")]
        MyReputationList(super::MyReputationList),
        #[prost(message, tag = "26")]
        MyProofOfBurnList(super::MyProofOfBurnList),
        #[prost(message, tag = "27")]
        UnconfirmedBsqChangeOutputList(super::UnconfirmedBsqChangeOutputList),
        #[prost(message, tag = "28")]
        SignedWitnessStore(super::SignedWitnessStore),
        #[prost(message, tag = "29")]
        MediationDisputeList(super::MediationDisputeList),
    }
}
///////////////////////////////////////////////////////////////////////////////////////////
// Collections
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceNumberMap {
    #[prost(message, repeated, tag = "1")]
    pub sequence_number_entries: ::std::vec::Vec<SequenceNumberEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceNumberEntry {
    #[prost(message, optional, tag = "1")]
    pub bytes: ::std::option::Option<ByteArray>,
    #[prost(message, optional, tag = "2")]
    pub map_value: ::std::option::Option<MapValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteArray {
    #[prost(bytes, tag = "1")]
    pub bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapValue {
    #[prost(int32, tag = "1")]
    pub sequence_nr: i32,
    #[prost(int64, tag = "2")]
    pub time_stamp: i64,
}
/// deprecated. Not used anymore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistedEntryMap {
    #[prost(map = "string, message", tag = "1")]
    pub persisted_entry_map:
        ::std::collections::HashMap<std::string::String, ProtectedStorageEntry>,
}
/// deprecated. Not used anymore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistableNetworkPayloadList {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<PersistableNetworkPayload>,
}
/// We use a list not a hash map to save disc space. The hash can be calculated from the payload anyway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAgeWitnessStore {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<AccountAgeWitness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedWitnessStore {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<SignedWitness>,
}
/// We use a list not a hash map to save disc space. The hash can be calculated from the payload anyway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeStatistics2Store {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<TradeStatistics2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerList {
    #[prost(message, repeated, tag = "1")]
    pub peer: ::std::vec::Vec<Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressEntryList {
    #[prost(message, repeated, tag = "1")]
    pub address_entry: ::std::vec::Vec<AddressEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressEntry {
    #[prost(string, tag = "7")]
    pub offer_id: std::string::String,
    #[prost(enumeration = "address_entry::Context", tag = "8")]
    pub context: i32,
    #[prost(bytes, tag = "9")]
    pub pub_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "10")]
    pub pub_key_hash: std::vec::Vec<u8>,
    #[prost(int64, tag = "11")]
    pub coin_locked_in_multi_sig: i64,
}
pub mod address_entry {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Context {
        PbError = 0,
        Arbitrator = 1,
        Available = 2,
        OfferFunding = 3,
        ReservedForTrade = 4,
        MultiSig = 5,
        TradePayout = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationPath {
    #[prost(string, repeated, tag = "1")]
    pub path: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccountList {
    #[prost(message, repeated, tag = "1")]
    pub payment_account: ::std::vec::Vec<PaymentAccount>,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Offer/Trade
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradableList {
    #[prost(message, repeated, tag = "1")]
    pub tradable: ::std::vec::Vec<Tradable>,
}
/// deprecated  Was used in pre v0.6.0 version. Not used anymore but leave it as it is used in PersistableEnvelope
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeStatisticsList {
    #[prost(message, repeated, tag = "1")]
    pub trade_statistics: ::std::vec::Vec<TradeStatistics>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offer {
    #[prost(message, optional, tag = "1")]
    pub offer_payload: ::std::option::Option<OfferPayload>,
}
pub mod offer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        PbError = 0,
        Unknown = 1,
        OfferFeePaid = 2,
        Available = 3,
        NotAvailable = 4,
        Removed = 5,
        MakerOffline = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenOffer {
    #[prost(message, optional, tag = "1")]
    pub offer: ::std::option::Option<Offer>,
    #[prost(enumeration = "open_offer::State", tag = "2")]
    pub state: i32,
    #[prost(message, optional, tag = "3")]
    pub arbitrator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "4")]
    pub mediator_node_address: ::std::option::Option<NodeAddress>,
}
pub mod open_offer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        PbError = 0,
        Available = 1,
        Reserved = 2,
        Closed = 3,
        Canceled = 4,
        Deactivated = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tradable {
    #[prost(oneof = "tradable::Message", tags = "1, 2, 3, 4, 5")]
    pub message: ::std::option::Option<tradable::Message>,
}
pub mod tradable {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        OpenOffer(super::OpenOffer),
        #[prost(message, tag = "2")]
        BuyerAsMakerTrade(super::BuyerAsMakerTrade),
        #[prost(message, tag = "3")]
        BuyerAsTakerTrade(super::BuyerAsTakerTrade),
        #[prost(message, tag = "4")]
        SellerAsMakerTrade(super::SellerAsMakerTrade),
        #[prost(message, tag = "5")]
        SellerAsTakerTrade(super::SellerAsTakerTrade),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(message, optional, tag = "1")]
    pub offer: ::std::option::Option<Offer>,
    #[prost(message, optional, tag = "2")]
    pub process_model: ::std::option::Option<ProcessModel>,
    #[prost(string, tag = "3")]
    pub taker_fee_tx_id: std::string::String,
    #[prost(string, tag = "4")]
    pub deposit_tx_id: std::string::String,
    #[prost(string, tag = "5")]
    pub payout_tx_id: std::string::String,
    #[prost(int64, tag = "6")]
    pub trade_amount_as_long: i64,
    #[prost(int64, tag = "7")]
    pub tx_fee_as_long: i64,
    #[prost(int64, tag = "8")]
    pub taker_fee_as_long: i64,
    #[prost(int64, tag = "9")]
    pub take_offer_date: i64,
    #[prost(bool, tag = "10")]
    pub is_currency_for_taker_fee_btc: bool,
    #[prost(int64, tag = "11")]
    pub trade_price: i64,
    #[prost(message, optional, tag = "12")]
    pub trading_peer_node_address: ::std::option::Option<NodeAddress>,
    #[prost(enumeration = "trade::State", tag = "13")]
    pub state: i32,
    #[prost(enumeration = "trade::DisputeState", tag = "14")]
    pub dispute_state: i32,
    #[prost(enumeration = "trade::TradePeriodState", tag = "15")]
    pub trade_period_state: i32,
    #[prost(message, optional, tag = "16")]
    pub contract: ::std::option::Option<Contract>,
    #[prost(string, tag = "17")]
    pub contract_as_json: std::string::String,
    #[prost(bytes, tag = "18")]
    pub contract_hash: std::vec::Vec<u8>,
    #[prost(string, tag = "19")]
    pub taker_contract_signature: std::string::String,
    #[prost(string, tag = "20")]
    pub maker_contract_signature: std::string::String,
    #[prost(message, optional, tag = "21")]
    pub arbitrator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(message, optional, tag = "22")]
    pub mediator_node_address: ::std::option::Option<NodeAddress>,
    #[prost(bytes, tag = "23")]
    pub arbitrator_btc_pub_key: std::vec::Vec<u8>,
    #[prost(string, tag = "24")]
    pub taker_payment_account_id: std::string::String,
    #[prost(string, tag = "25")]
    pub error_message: std::string::String,
    #[prost(message, optional, tag = "26")]
    pub arbitrator_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(message, optional, tag = "27")]
    pub mediator_pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(string, tag = "28")]
    pub counter_currency_tx_id: std::string::String,
    #[prost(message, repeated, tag = "29")]
    pub chat_message: ::std::vec::Vec<ChatMessage>,
    #[prost(enumeration = "MediationResultState", tag = "30")]
    pub mediation_result_state: i32,
}
pub mod trade {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        PbErrorState = 0,
        Preparation = 1,
        TakerPublishedTakerFeeTx = 2,
        MakerSentPublishDepositTxRequest = 3,
        MakerSawArrivedPublishDepositTxRequest = 4,
        MakerStoredInMailboxPublishDepositTxRequest = 5,
        MakerSendFailedPublishDepositTxRequest = 6,
        TakerReceivedPublishDepositTxRequest = 7,
        TakerPublishedDepositTx = 8,
        TakerSentDepositTxPublishedMsg = 9,
        TakerSawArrivedDepositTxPublishedMsg = 10,
        TakerStoredInMailboxDepositTxPublishedMsg = 11,
        TakerSendFailedDepositTxPublishedMsg = 12,
        MakerReceivedDepositTxPublishedMsg = 13,
        MakerSawDepositTxInNetwork = 14,
        DepositConfirmedInBlockChain = 15,
        BuyerConfirmedInUiFiatPaymentInitiated = 16,
        BuyerSentFiatPaymentInitiatedMsg = 17,
        BuyerSawArrivedFiatPaymentInitiatedMsg = 18,
        BuyerStoredInMailboxFiatPaymentInitiatedMsg = 19,
        BuyerSendFailedFiatPaymentInitiatedMsg = 20,
        SellerReceivedFiatPaymentInitiatedMsg = 21,
        SellerConfirmedInUiFiatPaymentReceipt = 22,
        SellerPublishedPayoutTx = 23,
        SellerSentPayoutTxPublishedMsg = 24,
        SellerSawArrivedPayoutTxPublishedMsg = 25,
        SellerStoredInMailboxPayoutTxPublishedMsg = 26,
        SellerSendFailedPayoutTxPublishedMsg = 27,
        BuyerReceivedPayoutTxPublishedMsg = 28,
        BuyerSawPayoutTxInNetwork = 29,
        WithdrawCompleted = 30,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Phase {
        PbErrorPhase = 0,
        Init = 1,
        TakerFeePublished = 2,
        DepositPublished = 3,
        DepositConfirmed = 4,
        FiatSent = 5,
        FiatReceived = 6,
        PayoutPublished = 7,
        Withdrawn = 8,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DisputeState {
        PbErrorDisputeState = 0,
        NoDispute = 1,
        /// arbitration  We use the enum name for resolving enums so it cannot be renamed
        DisputeRequested = 2,
        /// arbitration  We use the enum name for resolving enums so it cannot be renamed
        DisputeStartedByPeer = 3,
        /// arbitration  We use the enum name for resolving enums so it cannot be renamed
        DisputeClosed = 4,
        MediationRequested = 5,
        MediationStartedByPeer = 6,
        MediationClosed = 7,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TradePeriodState {
        PbErrorTradePeriodState = 0,
        FirstHalf = 1,
        SecondHalf = 2,
        TradePeriodOver = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyerAsMakerTrade {
    #[prost(message, optional, tag = "1")]
    pub trade: ::std::option::Option<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyerAsTakerTrade {
    #[prost(message, optional, tag = "1")]
    pub trade: ::std::option::Option<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellerAsMakerTrade {
    #[prost(message, optional, tag = "1")]
    pub trade: ::std::option::Option<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellerAsTakerTrade {
    #[prost(message, optional, tag = "1")]
    pub trade: ::std::option::Option<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessModel {
    #[prost(message, optional, tag = "1")]
    pub trading_peer: ::std::option::Option<TradingPeer>,
    #[prost(string, tag = "2")]
    pub offer_id: std::string::String,
    #[prost(string, tag = "3")]
    pub account_id: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(string, tag = "5")]
    pub take_offer_fee_tx_id: std::string::String,
    #[prost(bytes, tag = "6")]
    pub payout_tx_signature: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "7")]
    pub taker_accepted_arbitrator_node_addresses: ::std::vec::Vec<NodeAddress>,
    #[prost(message, repeated, tag = "8")]
    pub taker_accepted_mediator_node_addresses: ::std::vec::Vec<NodeAddress>,
    #[prost(bytes, tag = "9")]
    pub prepared_deposit_tx: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "10")]
    pub raw_transaction_inputs: ::std::vec::Vec<RawTransactionInput>,
    #[prost(int64, tag = "11")]
    pub change_output_value: i64,
    #[prost(string, tag = "12")]
    pub change_output_address: std::string::String,
    #[prost(bool, tag = "13")]
    pub use_savings_wallet: bool,
    #[prost(int64, tag = "14")]
    pub funds_needed_for_trade_as_long: i64,
    #[prost(bytes, tag = "15")]
    pub my_multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "16")]
    pub temp_trading_peer_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "17")]
    pub payment_started_message_state: std::string::String,
    #[prost(bytes, tag = "18")]
    pub mediated_payout_tx_signature: std::vec::Vec<u8>,
    #[prost(int64, tag = "19")]
    pub buyer_payout_amount_from_mediation: i64,
    #[prost(int64, tag = "20")]
    pub seller_payout_amount_from_mediation: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingPeer {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
    #[prost(string, tag = "3")]
    pub payout_address_string: std::string::String,
    #[prost(string, tag = "4")]
    pub contract_as_json: std::string::String,
    #[prost(string, tag = "5")]
    pub contract_signature: std::string::String,
    #[prost(bytes, tag = "6")]
    pub signature: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub pub_key_ring: ::std::option::Option<PubKeyRing>,
    #[prost(bytes, tag = "8")]
    pub multi_sig_pub_key: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "9")]
    pub raw_transaction_inputs: ::std::vec::Vec<RawTransactionInput>,
    #[prost(int64, tag = "10")]
    pub change_output_value: i64,
    #[prost(string, tag = "11")]
    pub change_output_address: std::string::String,
    #[prost(bytes, tag = "12")]
    pub account_age_witness_nonce: std::vec::Vec<u8>,
    #[prost(bytes, tag = "13")]
    pub account_age_witness_signature: std::vec::Vec<u8>,
    #[prost(int64, tag = "14")]
    pub current_date: i64,
    #[prost(bytes, tag = "15")]
    pub mediated_payout_tx_signature: std::vec::Vec<u8>,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Dispute
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArbitrationDisputeList {
    #[prost(message, repeated, tag = "1")]
    pub dispute: ::std::vec::Vec<Dispute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediationDisputeList {
    #[prost(message, repeated, tag = "1")]
    pub dispute: ::std::vec::Vec<Dispute>,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Preferences
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferencesPayload {
    #[prost(string, tag = "1")]
    pub user_language: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub user_country: ::std::option::Option<Country>,
    #[prost(message, repeated, tag = "3")]
    pub fiat_currencies: ::std::vec::Vec<TradeCurrency>,
    #[prost(message, repeated, tag = "4")]
    pub crypto_currencies: ::std::vec::Vec<TradeCurrency>,
    #[prost(message, optional, tag = "5")]
    pub block_chain_explorer_main_net: ::std::option::Option<BlockChainExplorer>,
    #[prost(message, optional, tag = "6")]
    pub block_chain_explorer_test_net: ::std::option::Option<BlockChainExplorer>,
    #[prost(message, optional, tag = "7")]
    pub bsq_block_chain_explorer: ::std::option::Option<BlockChainExplorer>,
    #[prost(string, tag = "8")]
    pub backup_directory: std::string::String,
    #[prost(bool, tag = "9")]
    pub auto_select_arbitrators: bool,
    #[prost(map = "string, bool", tag = "10")]
    pub dont_show_again_map: ::std::collections::HashMap<std::string::String, bool>,
    #[prost(bool, tag = "11")]
    pub tac_accepted: bool,
    #[prost(bool, tag = "12")]
    pub use_tor_for_bitcoin_j: bool,
    #[prost(bool, tag = "13")]
    pub show_own_offers_in_offer_book: bool,
    #[prost(message, optional, tag = "14")]
    pub preferred_trade_currency: ::std::option::Option<TradeCurrency>,
    #[prost(int64, tag = "15")]
    pub withdrawal_tx_fee_in_bytes: i64,
    #[prost(bool, tag = "16")]
    pub use_custom_withdrawal_tx_fee: bool,
    #[prost(double, tag = "17")]
    pub max_price_distance_in_percent: f64,
    #[prost(string, tag = "18")]
    pub offer_book_chart_screen_currency_code: std::string::String,
    #[prost(string, tag = "19")]
    pub trade_charts_screen_currency_code: std::string::String,
    #[prost(string, tag = "20")]
    pub buy_screen_currency_code: std::string::String,
    #[prost(string, tag = "21")]
    pub sell_screen_currency_code: std::string::String,
    #[prost(int32, tag = "22")]
    pub trade_statistics_tick_unit_index: i32,
    #[prost(bool, tag = "23")]
    pub resync_spv_requested: bool,
    #[prost(bool, tag = "24")]
    pub sort_market_currencies_numerically: bool,
    #[prost(bool, tag = "25")]
    pub use_percentage_based_price: bool,
    #[prost(map = "string, string", tag = "26")]
    pub peer_tag_map: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(string, tag = "27")]
    pub bitcoin_nodes: std::string::String,
    #[prost(string, repeated, tag = "28")]
    pub ignore_traders_list: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag = "29")]
    pub directory_chooser_path: std::string::String,
    /// Superseded by buyerSecurityDepositAsPercent
    #[prost(int64, tag = "30")]
    pub buyer_security_deposit_as_long: i64,
    #[prost(bool, tag = "31")]
    pub use_animations: bool,
    #[prost(message, optional, tag = "32")]
    pub selected_payment_account_for_create_offer: ::std::option::Option<PaymentAccount>,
    #[prost(bool, tag = "33")]
    pub pay_fee_in_btc: bool,
    #[prost(string, repeated, tag = "34")]
    pub bridge_addresses: ::std::vec::Vec<std::string::String>,
    #[prost(int32, tag = "35")]
    pub bridge_option_ordinal: i32,
    #[prost(int32, tag = "36")]
    pub tor_transport_ordinal: i32,
    #[prost(string, tag = "37")]
    pub custom_bridges: std::string::String,
    #[prost(int32, tag = "38")]
    pub bitcoin_nodes_option_ordinal: i32,
    #[prost(string, tag = "39")]
    pub referral_id: std::string::String,
    #[prost(string, tag = "40")]
    pub phone_key_and_token: std::string::String,
    #[prost(bool, tag = "41")]
    pub use_sound_for_mobile_notifications: bool,
    #[prost(bool, tag = "42")]
    pub use_trade_notifications: bool,
    #[prost(bool, tag = "43")]
    pub use_market_notifications: bool,
    #[prost(bool, tag = "44")]
    pub use_price_notifications: bool,
    #[prost(bool, tag = "45")]
    pub use_standby_mode: bool,
    #[prost(bool, tag = "46")]
    pub is_dao_full_node: bool,
    #[prost(string, tag = "47")]
    pub rpc_user: std::string::String,
    #[prost(string, tag = "48")]
    pub rpc_pw: std::string::String,
    #[prost(string, tag = "49")]
    pub take_offer_selected_payment_account_id: std::string::String,
    #[prost(double, tag = "50")]
    pub buyer_security_deposit_as_percent: f64,
    #[prost(int32, tag = "51")]
    pub ignore_dust_threshold: i32,
    #[prost(double, tag = "52")]
    pub buyer_security_deposit_as_percent_for_crypto: f64,
    #[prost(int32, tag = "53")]
    pub block_notify_port: i32,
    #[prost(int32, tag = "54")]
    pub css_theme: i32,
}
///////////////////////////////////////////////////////////////////////////////////////////
// UserPayload
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPayload {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
    #[prost(message, repeated, tag = "2")]
    pub payment_accounts: ::std::vec::Vec<PaymentAccount>,
    #[prost(message, optional, tag = "3")]
    pub current_payment_account: ::std::option::Option<PaymentAccount>,
    #[prost(string, repeated, tag = "4")]
    pub accepted_language_locale_codes: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "5")]
    pub developers_alert: ::std::option::Option<Alert>,
    #[prost(message, optional, tag = "6")]
    pub displayed_alert: ::std::option::Option<Alert>,
    #[prost(message, optional, tag = "7")]
    pub developers_filter: ::std::option::Option<Filter>,
    #[prost(message, repeated, tag = "8")]
    pub accepted_arbitrators: ::std::vec::Vec<Arbitrator>,
    #[prost(message, repeated, tag = "9")]
    pub accepted_mediators: ::std::vec::Vec<Mediator>,
    #[prost(message, optional, tag = "10")]
    pub registered_arbitrator: ::std::option::Option<Arbitrator>,
    #[prost(message, optional, tag = "11")]
    pub registered_mediator: ::std::option::Option<Mediator>,
    #[prost(message, optional, tag = "12")]
    pub price_alert_filter: ::std::option::Option<PriceAlertFilter>,
    #[prost(message, repeated, tag = "13")]
    pub market_alert_filters: ::std::vec::Vec<MarketAlertFilter>,
}
///////////////////////////////////////////////////////////////////////////////////////////
// DAO
///////////////////////////////////////////////////////////////////////////////////////////

// blockchain

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseBlock {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(int64, tag = "2")]
    pub time: i64,
    #[prost(string, tag = "3")]
    pub hash: std::string::String,
    #[prost(string, tag = "4")]
    pub previous_block_hash: std::string::String,
    #[prost(oneof = "base_block::Message", tags = "5, 6")]
    pub message: ::std::option::Option<base_block::Message>,
}
pub mod base_block {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "5")]
        RawBlock(super::RawBlock),
        #[prost(message, tag = "6")]
        Block(super::Block),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBlock {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "1")]
    pub raw_txs: ::std::vec::Vec<BaseTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "1")]
    pub txs: ::std::vec::Vec<BaseTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseTx {
    #[prost(string, tag = "1")]
    pub tx_version: std::string::String,
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    #[prost(int32, tag = "3")]
    pub block_height: i32,
    #[prost(string, tag = "4")]
    pub block_hash: std::string::String,
    #[prost(int64, tag = "5")]
    pub time: i64,
    #[prost(message, repeated, tag = "6")]
    pub tx_inputs: ::std::vec::Vec<TxInput>,
    #[prost(oneof = "base_tx::Message", tags = "7, 8")]
    pub message: ::std::option::Option<base_tx::Message>,
}
pub mod base_tx {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "7")]
        RawTx(super::RawTx),
        #[prost(message, tag = "8")]
        Tx(super::Tx),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTx {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "1")]
    pub raw_tx_outputs: ::std::vec::Vec<BaseTxOutput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "1")]
    pub tx_outputs: ::std::vec::Vec<BaseTxOutput>,
    #[prost(enumeration = "TxType", tag = "2")]
    pub tx_type: i32,
    #[prost(int64, tag = "3")]
    pub burnt_bsq: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(string, tag = "1")]
    pub connected_tx_output_tx_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub connected_tx_output_index: i32,
    #[prost(string, tag = "3")]
    pub pub_key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseTxOutput {
    #[prost(int32, tag = "1")]
    pub index: i32,
    #[prost(int64, tag = "2")]
    pub value: i64,
    #[prost(string, tag = "3")]
    pub tx_id: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub pub_key_script: ::std::option::Option<PubKeyScript>,
    #[prost(string, tag = "5")]
    pub address: std::string::String,
    #[prost(bytes, tag = "6")]
    pub op_return_data: std::vec::Vec<u8>,
    #[prost(int32, tag = "7")]
    pub block_height: i32,
    #[prost(oneof = "base_tx_output::Message", tags = "8, 9")]
    pub message: ::std::option::Option<base_tx_output::Message>,
}
pub mod base_tx_output {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "8")]
        RawTxOutput(super::RawTxOutput),
        #[prost(message, tag = "9")]
        TxOutput(super::TxOutput),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnconfirmedTxOutput {
    #[prost(int32, tag = "1")]
    pub index: i32,
    #[prost(int64, tag = "2")]
    pub value: i64,
    #[prost(string, tag = "3")]
    pub tx_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTxOutput {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(enumeration = "TxOutputType", tag = "1")]
    pub tx_output_type: i32,
    #[prost(int32, tag = "2")]
    pub lock_time: i32,
    #[prost(int32, tag = "3")]
    pub unlock_block_height: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpentInfo {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(string, tag = "2")]
    pub tx_id: std::string::String,
    #[prost(int32, tag = "3")]
    pub input_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyScript {
    #[prost(int32, tag = "1")]
    pub req_sigs: i32,
    #[prost(enumeration = "ScriptType", tag = "2")]
    pub script_type: i32,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag = "4")]
    pub asm: std::string::String,
    #[prost(string, tag = "5")]
    pub hex: std::string::String,
}
// dao data

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaoPhase {
    #[prost(int32, tag = "1")]
    pub phase_ordinal: i32,
    #[prost(int32, tag = "2")]
    pub duration: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cycle {
    #[prost(int32, tag = "1")]
    pub height_of_first_lock: i32,
    #[prost(message, repeated, tag = "2")]
    pub dao_phase: ::std::vec::Vec<DaoPhase>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaoState {
    #[prost(int32, tag = "1")]
    pub chain_height: i32,
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(message, repeated, tag = "2")]
    pub blocks: ::std::vec::Vec<BaseBlock>,
    #[prost(message, repeated, tag = "3")]
    pub cycles: ::std::vec::Vec<Cycle>,
    /// Because of the way how PB implements inheritence we need to use the super class as type
    #[prost(map = "string, message", tag = "4")]
    pub unspent_tx_output_map: ::std::collections::HashMap<std::string::String, BaseTxOutput>,
    #[prost(map = "string, message", tag = "5")]
    pub issuance_map: ::std::collections::HashMap<std::string::String, Issuance>,
    #[prost(string, repeated, tag = "6")]
    pub confiscated_lockup_tx_list: ::std::vec::Vec<std::string::String>,
    #[prost(map = "string, message", tag = "7")]
    pub spent_info_map: ::std::collections::HashMap<std::string::String, SpentInfo>,
    #[prost(message, repeated, tag = "8")]
    pub param_change_list: ::std::vec::Vec<ParamChange>,
    #[prost(message, repeated, tag = "9")]
    pub evaluated_proposal_list: ::std::vec::Vec<EvaluatedProposal>,
    #[prost(message, repeated, tag = "10")]
    pub decrypted_ballots_with_merits_list: ::std::vec::Vec<DecryptedBallotsWithMerits>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issuance {
    #[prost(string, tag = "1")]
    pub tx_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub chain_height: i32,
    #[prost(int64, tag = "3")]
    pub amount: i64,
    #[prost(string, tag = "4")]
    pub pub_key: std::string::String,
    #[prost(string, tag = "5")]
    pub issuance_type: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub link: std::string::String,
    #[prost(uint32, tag = "3")]
    pub version: u32,
    #[prost(int64, tag = "4")]
    pub creation_date: i64,
    #[prost(string, tag = "5")]
    pub tx_id: std::string::String,
    /// We leave some index space here in case we add more subclasses
    #[prost(map = "string, string", tag = "20")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(oneof = "proposal::Message", tags = "6, 7, 8, 9, 10, 11, 12")]
    pub message: ::std::option::Option<proposal::Message>,
}
pub mod proposal {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "6")]
        CompensationProposal(super::CompensationProposal),
        #[prost(message, tag = "7")]
        ReimbursementProposal(super::ReimbursementProposal),
        #[prost(message, tag = "8")]
        ChangeParamProposal(super::ChangeParamProposal),
        #[prost(message, tag = "9")]
        RoleProposal(super::RoleProposal),
        #[prost(message, tag = "10")]
        ConfiscateBondProposal(super::ConfiscateBondProposal),
        #[prost(message, tag = "11")]
        GenericProposal(super::GenericProposal),
        #[prost(message, tag = "12")]
        RemoveAssetProposal(super::RemoveAssetProposal),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompensationProposal {
    #[prost(int64, tag = "1")]
    pub requested_bsq: i64,
    #[prost(string, tag = "2")]
    pub bsq_address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReimbursementProposal {
    #[prost(int64, tag = "1")]
    pub requested_bsq: i64,
    #[prost(string, tag = "2")]
    pub bsq_address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeParamProposal {
    /// name of enum
    #[prost(string, tag = "1")]
    pub param: std::string::String,
    #[prost(string, tag = "2")]
    pub param_value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleProposal {
    #[prost(message, optional, tag = "1")]
    pub role: ::std::option::Option<Role>,
    #[prost(int64, tag = "2")]
    pub required_bond_unit: i64,
    #[prost(int32, tag = "3")]
    pub unlock_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfiscateBondProposal {
    #[prost(string, tag = "1")]
    pub lockup_tx_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericProposal {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAssetProposal {
    #[prost(string, tag = "1")]
    pub ticker_symbol: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(string, tag = "3")]
    pub link: std::string::String,
    /// name of BondedRoleType enum
    #[prost(string, tag = "4")]
    pub bonded_role_type: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyReputation {
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    #[prost(bytes, tag = "2")]
    pub salt: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyReputationList {
    #[prost(message, repeated, tag = "1")]
    pub my_reputation: ::std::vec::Vec<MyReputation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyProofOfBurn {
    #[prost(string, tag = "1")]
    pub tx_id: std::string::String,
    #[prost(string, tag = "2")]
    pub pre_image: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyProofOfBurnList {
    #[prost(message, repeated, tag = "1")]
    pub my_proof_of_burn: ::std::vec::Vec<MyProofOfBurn>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnconfirmedBsqChangeOutputList {
    #[prost(message, repeated, tag = "1")]
    pub unconfirmed_tx_output: ::std::vec::Vec<UnconfirmedTxOutput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TempProposalPayload {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::std::option::Option<Proposal>,
    #[prost(bytes, tag = "2")]
    pub owner_pub_key_encoded: std::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "3")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalPayload {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::std::option::Option<Proposal>,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalStore {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<ProposalPayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TempProposalStore {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<ProtectedStorageEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ballot {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::std::option::Option<Proposal>,
    #[prost(message, optional, tag = "2")]
    pub vote: ::std::option::Option<Vote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyProposalList {
    #[prost(message, repeated, tag = "1")]
    pub proposal: ::std::vec::Vec<Proposal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BallotList {
    #[prost(message, repeated, tag = "1")]
    pub ballot: ::std::vec::Vec<Ballot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamChange {
    #[prost(string, tag = "1")]
    pub param_name: std::string::String,
    #[prost(string, tag = "2")]
    pub param_value: std::string::String,
    #[prost(int32, tag = "3")]
    pub activation_height: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfiscateBond {
    #[prost(string, tag = "1")]
    pub lockup_tx_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyVote {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(message, optional, tag = "2")]
    pub ballot_list: ::std::option::Option<BallotList>,
    #[prost(bytes, tag = "3")]
    pub secret_key_encoded: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub blind_vote: ::std::option::Option<BlindVote>,
    #[prost(int64, tag = "5")]
    pub date: i64,
    #[prost(string, tag = "6")]
    pub reveal_tx_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyVoteList {
    #[prost(message, repeated, tag = "1")]
    pub my_vote: ::std::vec::Vec<MyVote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteWithProposalTxId {
    #[prost(string, tag = "1")]
    pub proposal_tx_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub vote: ::std::option::Option<Vote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteWithProposalTxIdList {
    #[prost(message, repeated, tag = "1")]
    pub item: ::std::vec::Vec<VoteWithProposalTxId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlindVote {
    #[prost(bytes, tag = "1")]
    pub encrypted_votes: std::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub tx_id: std::string::String,
    #[prost(int64, tag = "3")]
    pub stake: i64,
    #[prost(bytes, tag = "4")]
    pub encrypted_merit_list: std::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    pub date: i64,
    #[prost(map = "string, string", tag = "6")]
    pub extra_data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyBlindVoteList {
    #[prost(message, repeated, tag = "1")]
    pub blind_vote: ::std::vec::Vec<BlindVote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlindVoteStore {
    #[prost(message, repeated, tag = "1")]
    pub items: ::std::vec::Vec<BlindVotePayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlindVotePayload {
    #[prost(message, optional, tag = "1")]
    pub blind_vote: ::std::option::Option<BlindVote>,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(bool, tag = "1")]
    pub accepted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Merit {
    #[prost(message, optional, tag = "1")]
    pub issuance: ::std::option::Option<Issuance>,
    #[prost(bytes, tag = "2")]
    pub signature: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeritList {
    #[prost(message, repeated, tag = "1")]
    pub merit: ::std::vec::Vec<Merit>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalVoteResult {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::std::option::Option<Proposal>,
    #[prost(int64, tag = "2")]
    pub stake_of_accepted_votes: i64,
    #[prost(int64, tag = "3")]
    pub stake_of_rejected_votes: i64,
    #[prost(int32, tag = "4")]
    pub num_accepted_votes: i32,
    #[prost(int32, tag = "5")]
    pub num_rejected_votes: i32,
    #[prost(int32, tag = "6")]
    pub num_ignored_votes: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatedProposal {
    #[prost(bool, tag = "1")]
    pub is_accepted: bool,
    #[prost(message, optional, tag = "2")]
    pub proposal_vote_result: ::std::option::Option<ProposalVoteResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptedBallotsWithMerits {
    #[prost(bytes, tag = "1")]
    pub hash_of_blind_vote_list: std::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub blind_vote_tx_id: std::string::String,
    #[prost(string, tag = "3")]
    pub vote_reveal_tx_id: std::string::String,
    #[prost(int64, tag = "4")]
    pub stake: i64,
    #[prost(message, optional, tag = "5")]
    pub ballot_list: ::std::option::Option<BallotList>,
    #[prost(message, optional, tag = "6")]
    pub merit_list: ::std::option::Option<MeritList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaoStateStore {
    #[prost(message, optional, tag = "1")]
    pub dao_state: ::std::option::Option<DaoState>,
    #[prost(message, repeated, tag = "2")]
    pub dao_state_hash: ::std::vec::Vec<DaoStateHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaoStateHash {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub prev_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalStateHash {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub prev_hash: std::vec::Vec<u8>,
    #[prost(int32, tag = "4")]
    pub num_proposals: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlindVoteStateHash {
    #[prost(int32, tag = "1")]
    pub height: i32,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub prev_hash: std::vec::Vec<u8>,
    #[prost(int32, tag = "4")]
    pub num_blind_votes: i32,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Misc
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockChainExplorer {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub tx_url: std::string::String,
    #[prost(string, tag = "3")]
    pub address_url: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccount {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(int64, tag = "2")]
    pub creation_date: i64,
    #[prost(message, optional, tag = "3")]
    pub payment_method: ::std::option::Option<PaymentMethod>,
    #[prost(string, tag = "4")]
    pub account_name: std::string::String,
    #[prost(message, repeated, tag = "5")]
    pub trade_currencies: ::std::vec::Vec<TradeCurrency>,
    #[prost(message, optional, tag = "6")]
    pub selected_trade_currency: ::std::option::Option<TradeCurrency>,
    #[prost(message, optional, tag = "7")]
    pub payment_account_payload: ::std::option::Option<PaymentAccountPayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentMethod {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(int64, tag = "2")]
    pub max_trade_period: i64,
    #[prost(int64, tag = "3")]
    pub max_trade_limit: i64,
}
// Currency

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    #[prost(string, tag = "1")]
    pub currency_code: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeCurrency {
    #[prost(string, tag = "1")]
    pub code: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(oneof = "trade_currency::Message", tags = "3, 4")]
    pub message: ::std::option::Option<trade_currency::Message>,
}
pub mod trade_currency {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "3")]
        CryptoCurrency(super::CryptoCurrency),
        #[prost(message, tag = "4")]
        FiatCurrency(super::FiatCurrency),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoCurrency {
    #[prost(bool, tag = "1")]
    pub is_asset: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FiatCurrency {
    #[prost(message, optional, tag = "1")]
    pub currency: ::std::option::Option<Currency>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Country {
    #[prost(string, tag = "1")]
    pub code: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub region: ::std::option::Option<Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(string, tag = "1")]
    pub code: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Notifications
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceAlertFilter {
    #[prost(string, tag = "1")]
    pub currency_code: std::string::String,
    #[prost(int64, tag = "2")]
    pub high: i64,
    #[prost(int64, tag = "3")]
    pub low: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketAlertFilter {
    #[prost(message, optional, tag = "1")]
    pub payment_account: ::std::option::Option<PaymentAccount>,
    #[prost(int32, tag = "2")]
    pub trigger_value: i32,
    #[prost(bool, tag = "3")]
    pub is_buy_offer: bool,
    #[prost(string, repeated, tag = "4")]
    pub alert_ids: ::std::vec::Vec<std::string::String>,
}
///////////////////////////////////////////////////////////////////////////////////////////
// Mock
///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MockMailboxPayload {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub sender_node_address: ::std::option::Option<NodeAddress>,
    #[prost(string, tag = "3")]
    pub uid: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MockPayload {
    #[prost(string, tag = "1")]
    pub message_version: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
// dispute

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SupportType {
    Arbitration = 0,
    Mediation = 1,
    Trade = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvailabilityResult {
    PbError = 0,
    UnknownFailure = 1,
    Available = 2,
    OfferTaken = 3,
    PriceOutOfTolerance = 4,
    MarketPriceNotAvailable = 5,
    NoArbitrators = 6,
    NoMediators = 7,
    UserIgnored = 8,
    MissingMandatoryCapability = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediationResultState {
    PbErrorMediationResult = 0,
    UndefinedMediationResult = 1,
    MediationResultAccepted = 2,
    MediationResultRejected = 3,
    SigMsgSent = 4,
    SigMsgArrived = 5,
    SigMsgInMailbox = 6,
    SigMsgSendFailed = 7,
    ReceivedSigMsg = 8,
    PayoutTxPublished = 9,
    PayoutTxPublishedMsgSent = 10,
    PayoutTxPublishedMsgArrived = 11,
    PayoutTxPublishedMsgInMailbox = 12,
    PayoutTxPublishedMsgSendFailed = 13,
    ReceivedPayoutTxPublishedMsg = 14,
    PayoutTxSeenInNetwork = 15,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxType {
    PbErrorTxType = 0,
    UndefinedTxType = 1,
    Unverified = 2,
    Invalid = 3,
    Genesis = 4,
    TransferBsq = 5,
    PayTradeFee = 6,
    Proposal = 7,
    CompensationRequest = 8,
    ReimbursementRequest = 9,
    BlindVote = 10,
    VoteReveal = 11,
    Lockup = 12,
    Unlock = 13,
    AssetListingFee = 14,
    ProofOfBurn = 15,
    Irregular = 16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxOutputType {
    PbErrorTxOutputType = 0,
    UndefinedOutput = 1,
    GenesisOutput = 2,
    BsqOutput = 3,
    BtcOutput = 4,
    ProposalOpReturnOutput = 5,
    CompReqOpReturnOutput = 6,
    ReimbursementOpReturnOutput = 7,
    ConfiscateBondOpReturnOutput = 8,
    IssuanceCandidateOutput = 9,
    BlindVoteLockStakeOutput = 10,
    BlindVoteOpReturnOutput = 11,
    VoteRevealUnlockStakeOutput = 12,
    VoteRevealOpReturnOutput = 13,
    AssetListingFeeOpReturnOutput = 14,
    ProofOfBurnOpReturnOutput = 15,
    LockupOutput = 16,
    LockupOpReturnOutput = 17,
    UnlockOutput = 18,
    InvalidOutput = 19,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScriptType {
    PbErrorScriptTypes = 0,
    PubKey = 1,
    PubKeyHash = 2,
    ScriptHash = 3,
    Multisig = 4,
    NullData = 5,
    WitnessV0Keyhash = 6,
    WitnessV0Scripthash = 7,
    Nonstandard = 8,
}
