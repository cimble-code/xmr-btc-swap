use crate::monero;
use crate::network::cbor_request_response::CborCodec;
use crate::protocol::{alice, bob};
use libp2p::core::ProtocolName;
use libp2p::request_response::{
    ProtocolSupport, RequestResponse, RequestResponseConfig, RequestResponseEvent,
    RequestResponseMessage,
};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

const PROTOCOL: &str = "/comit/xmr/btc/transfer_proof/1.0.0";
type OutEvent = RequestResponseEvent<Request, ()>;
type Message = RequestResponseMessage<Request, ()>;

pub type Behaviour = RequestResponse<CborCodec<TransferProofProtocol, Request, ()>>;

#[derive(Debug, Clone, Copy, Default)]
pub struct TransferProofProtocol;

impl ProtocolName for TransferProofProtocol {
    fn protocol_name(&self) -> &[u8] {
        PROTOCOL.as_bytes()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub tx_lock_proof: monero::TransferProof,
}

pub fn alice() -> Behaviour {
    Behaviour::new(
        CborCodec::default(),
        vec![(TransferProofProtocol, ProtocolSupport::Outbound)],
        RequestResponseConfig::default(),
    )
}

pub fn bob() -> Behaviour {
    Behaviour::new(
        CborCodec::default(),
        vec![(TransferProofProtocol, ProtocolSupport::Inbound)],
        RequestResponseConfig::default(),
    )
}

impl From<(PeerId, Message)> for alice::OutEvent {
    fn from((peer, message): (PeerId, Message)) -> Self {
        match message {
            Message::Request { .. } => Self::unexpected_request(peer),
            Message::Response { request_id, .. } => Self::TransferProofAcknowledged {
                peer,
                id: request_id,
            },
        }
    }
}
crate::impl_from_rr_event!(OutEvent, alice::OutEvent, PROTOCOL);

impl From<(PeerId, Message)> for bob::OutEvent {
    fn from((peer, message): (PeerId, Message)) -> Self {
        match message {
            Message::Request {
                request, channel, ..
            } => Self::TransferProofReceived {
                msg: Box::new(request),
                channel,
            },
            Message::Response { .. } => Self::unexpected_response(peer),
        }
    }
}
crate::impl_from_rr_event!(OutEvent, bob::OutEvent, PROTOCOL);
