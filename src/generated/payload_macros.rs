macro_rules! for_all_payloads {
    ($m:ident) => {
        $m!(PreliminaryGetDataRequest,preliminary_get_data_request);
        $m!(GetDataResponse,get_data_response);
        $m!(GetUpdatedDataRequest,get_updated_data_request);
        $m!(GetPeersRequest,get_peers_request);
        $m!(GetPeersResponse,get_peers_response);
        $m!(Ping,ping);
        $m!(Pong,pong);
        $m!(OfferAvailabilityRequest,offer_availability_request);
        $m!(OfferAvailabilityResponse,offer_availability_response);
        $m!(RefreshOfferMessage,refresh_offer_message);
        $m!(AddDataMessage,add_data_message);
        $m!(RemoveDataMessage,remove_data_message);
        $m!(RemoveMailboxDataMessage,remove_mailbox_data_message);
        $m!(CloseConnectionMessage,close_connection_message);
        $m!(PrefixedSealedAndSignedMessage,prefixed_sealed_and_signed_message);
        $m!(PayDepositRequest,pay_deposit_request);
        $m!(PublishDepositTxRequest,publish_deposit_tx_request);
        $m!(DepositTxPublishedMessage,deposit_tx_published_message);
        $m!(CounterCurrencyTransferStartedMessage,counter_currency_transfer_started_message);
        $m!(PayoutTxPublishedMessage,payout_tx_published_message);
        $m!(OpenNewDisputeMessage,open_new_dispute_message);
        $m!(PeerOpenedDisputeMessage,peer_opened_dispute_message);
        $m!(ChatMessage,chat_message);
        $m!(DisputeResultMessage,dispute_result_message);
        $m!(PeerPublishedDisputePayoutTxMessage,peer_published_dispute_payout_tx_message);
        $m!(PrivateNotificationMessage,private_notification_message);
        $m!(GetBlocksRequest,get_blocks_request);
        $m!(GetBlocksResponse,get_blocks_response);
        $m!(NewBlockBroadcastMessage,new_block_broadcast_message);
        $m!(AddPersistableNetworkPayloadMessage,add_persistable_network_payload_message);
        $m!(AckMessage,ack_message);
        $m!(RepublishGovernanceDataRequest,republish_governance_data_request);
        $m!(NewDaoStateHashMessage,new_dao_state_hash_message);
        $m!(GetDaoStateHashesRequest,get_dao_state_hashes_request);
        $m!(GetDaoStateHashesResponse,get_dao_state_hashes_response);
        $m!(NewProposalStateHashMessage,new_proposal_state_hash_message);
        $m!(GetProposalStateHashesRequest,get_proposal_state_hashes_request);
        $m!(GetProposalStateHashesResponse,get_proposal_state_hashes_response);
        $m!(NewBlindVoteStateHashMessage,new_blind_vote_state_hash_message);
        $m!(GetBlindVoteStateHashesRequest,get_blind_vote_state_hashes_request);
        $m!(GetBlindVoteStateHashesResponse,get_blind_vote_state_hashes_response);
        $m!(BundleOfEnvelopes,bundle_of_envelopes);
        $m!(MediatedPayoutTxSignatureMessage,mediated_payout_tx_signature_message);
        $m!(MediatedPayoutTxPublishedMessage,mediated_payout_tx_published_message);
    };
}

macro_rules! match_payload {
    ($m:ident, $target:ident) => {
        match $m {
            network_envelope::Message::PreliminaryGetDataRequest($m) => $target.preliminary_get_data_request($m),
            network_envelope::Message::GetDataResponse($m) => $target.get_data_response($m),
            network_envelope::Message::GetUpdatedDataRequest($m) => $target.get_updated_data_request($m),
            network_envelope::Message::GetPeersRequest($m) => $target.get_peers_request($m),
            network_envelope::Message::GetPeersResponse($m) => $target.get_peers_response($m),
            network_envelope::Message::Ping($m) => $target.ping($m),
            network_envelope::Message::Pong($m) => $target.pong($m),
            network_envelope::Message::OfferAvailabilityRequest($m) => $target.offer_availability_request($m),
            network_envelope::Message::OfferAvailabilityResponse($m) => $target.offer_availability_response($m),
            network_envelope::Message::RefreshOfferMessage($m) => $target.refresh_offer_message($m),
            network_envelope::Message::AddDataMessage($m) => $target.add_data_message($m),
            network_envelope::Message::RemoveDataMessage($m) => $target.remove_data_message($m),
            network_envelope::Message::RemoveMailboxDataMessage($m) => $target.remove_mailbox_data_message($m),
            network_envelope::Message::CloseConnectionMessage($m) => $target.close_connection_message($m),
            network_envelope::Message::PrefixedSealedAndSignedMessage($m) => $target.prefixed_sealed_and_signed_message($m),
            network_envelope::Message::PayDepositRequest($m) => $target.pay_deposit_request($m),
            network_envelope::Message::PublishDepositTxRequest($m) => $target.publish_deposit_tx_request($m),
            network_envelope::Message::DepositTxPublishedMessage($m) => $target.deposit_tx_published_message($m),
            network_envelope::Message::CounterCurrencyTransferStartedMessage($m) => $target.counter_currency_transfer_started_message($m),
            network_envelope::Message::PayoutTxPublishedMessage($m) => $target.payout_tx_published_message($m),
            network_envelope::Message::OpenNewDisputeMessage($m) => $target.open_new_dispute_message($m),
            network_envelope::Message::PeerOpenedDisputeMessage($m) => $target.peer_opened_dispute_message($m),
            network_envelope::Message::ChatMessage($m) => $target.chat_message($m),
            network_envelope::Message::DisputeResultMessage($m) => $target.dispute_result_message($m),
            network_envelope::Message::PeerPublishedDisputePayoutTxMessage($m) => $target.peer_published_dispute_payout_tx_message($m),
            network_envelope::Message::PrivateNotificationMessage($m) => $target.private_notification_message($m),
            network_envelope::Message::GetBlocksRequest($m) => $target.get_blocks_request($m),
            network_envelope::Message::GetBlocksResponse($m) => $target.get_blocks_response($m),
            network_envelope::Message::NewBlockBroadcastMessage($m) => $target.new_block_broadcast_message($m),
            network_envelope::Message::AddPersistableNetworkPayloadMessage($m) => $target.add_persistable_network_payload_message($m),
            network_envelope::Message::AckMessage($m) => $target.ack_message($m),
            network_envelope::Message::RepublishGovernanceDataRequest($m) => $target.republish_governance_data_request($m),
            network_envelope::Message::NewDaoStateHashMessage($m) => $target.new_dao_state_hash_message($m),
            network_envelope::Message::GetDaoStateHashesRequest($m) => $target.get_dao_state_hashes_request($m),
            network_envelope::Message::GetDaoStateHashesResponse($m) => $target.get_dao_state_hashes_response($m),
            network_envelope::Message::NewProposalStateHashMessage($m) => $target.new_proposal_state_hash_message($m),
            network_envelope::Message::GetProposalStateHashesRequest($m) => $target.get_proposal_state_hashes_request($m),
            network_envelope::Message::GetProposalStateHashesResponse($m) => $target.get_proposal_state_hashes_response($m),
            network_envelope::Message::NewBlindVoteStateHashMessage($m) => $target.new_blind_vote_state_hash_message($m),
            network_envelope::Message::GetBlindVoteStateHashesRequest($m) => $target.get_blind_vote_state_hashes_request($m),
            network_envelope::Message::GetBlindVoteStateHashesResponse($m) => $target.get_blind_vote_state_hashes_response($m),
            network_envelope::Message::BundleOfEnvelopes($m) => $target.bundle_of_envelopes($m),
            network_envelope::Message::MediatedPayoutTxSignatureMessage($m) => $target.mediated_payout_tx_signature_message($m),
            network_envelope::Message::MediatedPayoutTxPublishedMessage($m) => $target.mediated_payout_tx_published_message($m),
        }
    };
}
