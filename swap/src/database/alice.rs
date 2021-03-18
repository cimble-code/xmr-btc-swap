use crate::bitcoin::EncryptedSignature;
use crate::monero;
use crate::monero::monero_private_key;
use crate::protocol::alice;
use crate::protocol::alice::AliceState;
use ::bitcoin::hashes::core::fmt::Display;
use monero_rpc::wallet::BlockHeight;
use serde::{Deserialize, Serialize};

// Large enum variant is fine because this is only used for database
// and is dropped once written in DB.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Alice {
    WatchingForTxLockInMempool {
        state3: alice::State3,
    },
    WaitingForTxLockConfirmations {
        state3: alice::State3,
    },
    WaitingForEncSig {
        state3: alice::State3,
    },
    EncSigLearned {
        monero_wallet_restore_blockheight: BlockHeight,
        encrypted_signature: EncryptedSignature,
        state3: alice::State3,
    },
    CancelTimelockExpired {
        monero_wallet_restore_blockheight: BlockHeight,
        state3: alice::State3,
    },
    BtcCancelled {
        monero_wallet_restore_blockheight: BlockHeight,
        state3: alice::State3,
    },
    BtcPunishable {
        monero_wallet_restore_blockheight: BlockHeight,
        state3: alice::State3,
    },
    BtcRefunded {
        monero_wallet_restore_blockheight: BlockHeight,
        state3: alice::State3,
        #[serde(with = "monero_private_key")]
        spend_key: monero::PrivateKey,
    },
    Done(AliceEndState),
}

#[derive(Copy, Clone, strum::Display, Debug, Deserialize, Serialize, PartialEq)]
pub enum AliceEndState {
    SafelyAborted,
    BtcRedeemed,
    XmrRefunded,
    BtcPunished,
}

impl From<&AliceState> for Alice {
    fn from(alice_state: &AliceState) -> Self {
        match alice_state {
            AliceState::WatchingForTxLockInMempool { state3 } => {
                Alice::WatchingForTxLockInMempool {
                    state3: state3.as_ref().clone(),
                }
            }
            AliceState::WaitingForTxLockConfirmations { state3 } => {
                Alice::WaitingForTxLockConfirmations {
                    state3: state3.as_ref().clone(),
                }
            }
            AliceState::WaitingForEncSig { state3 } => Alice::WaitingForEncSig {
                state3: state3.as_ref().clone(),
            },
            AliceState::EncSigLearned {
                monero_wallet_restore_blockheight,
                state3,
                encrypted_signature,
            } => Alice::EncSigLearned {
                monero_wallet_restore_blockheight: *monero_wallet_restore_blockheight,
                state3: state3.as_ref().clone(),
                encrypted_signature: *encrypted_signature.clone(),
            },
            AliceState::BtcRedeemed => Alice::Done(AliceEndState::BtcRedeemed),
            AliceState::BtcCancelled {
                monero_wallet_restore_blockheight,
                state3,
                ..
            } => Alice::BtcCancelled {
                monero_wallet_restore_blockheight: *monero_wallet_restore_blockheight,
                state3: state3.as_ref().clone(),
            },
            AliceState::BtcRefunded {
                monero_wallet_restore_blockheight,
                spend_key,
                state3,
            } => Alice::BtcRefunded {
                monero_wallet_restore_blockheight: *monero_wallet_restore_blockheight,
                spend_key: *spend_key,
                state3: state3.as_ref().clone(),
            },
            AliceState::BtcPunishable {
                monero_wallet_restore_blockheight,
                state3,
                ..
            } => Alice::BtcPunishable {
                monero_wallet_restore_blockheight: *monero_wallet_restore_blockheight,
                state3: state3.as_ref().clone(),
            },
            AliceState::XmrRefunded => Alice::Done(AliceEndState::XmrRefunded),
            AliceState::CancelTimelockExpired {
                monero_wallet_restore_blockheight,
                state3,
            } => Alice::CancelTimelockExpired {
                monero_wallet_restore_blockheight: *monero_wallet_restore_blockheight,
                state3: state3.as_ref().clone(),
            },
            AliceState::BtcPunished => Alice::Done(AliceEndState::BtcPunished),
            AliceState::SafelyAborted => Alice::Done(AliceEndState::SafelyAborted),
        }
    }
}

impl From<Alice> for AliceState {
    fn from(db_state: Alice) -> Self {
        match db_state {
            Alice::WatchingForTxLockInMempool { state3 } => {
                AliceState::WatchingForTxLockInMempool {
                    state3: Box::new(state3),
                }
            }
            Alice::WaitingForTxLockConfirmations { state3 } => {
                AliceState::WatchingForTxLockInMempool {
                    state3: Box::new(state3),
                }
            }
            Alice::WaitingForEncSig { state3 } => AliceState::WaitingForEncSig {
                state3: Box::new(state3),
            },
            Alice::EncSigLearned {
                monero_wallet_restore_blockheight,
                state3: state,
                encrypted_signature,
            } => AliceState::EncSigLearned {
                monero_wallet_restore_blockheight,
                state3: Box::new(state),
                encrypted_signature: Box::new(encrypted_signature),
            },
            Alice::CancelTimelockExpired {
                monero_wallet_restore_blockheight,
                state3,
            } => AliceState::CancelTimelockExpired {
                monero_wallet_restore_blockheight,
                state3: Box::new(state3),
            },
            Alice::BtcCancelled {
                monero_wallet_restore_blockheight,
                state3,
            } => AliceState::BtcCancelled {
                monero_wallet_restore_blockheight,
                state3: Box::new(state3),
            },

            Alice::BtcPunishable {
                monero_wallet_restore_blockheight,
                state3,
            } => AliceState::BtcPunishable {
                monero_wallet_restore_blockheight,
                state3: Box::new(state3),
            },
            Alice::BtcRefunded {
                monero_wallet_restore_blockheight,
                state3,
                spend_key,
            } => AliceState::BtcRefunded {
                monero_wallet_restore_blockheight,
                spend_key,
                state3: Box::new(state3),
            },
            Alice::Done(end_state) => match end_state {
                AliceEndState::SafelyAborted => AliceState::SafelyAborted,
                AliceEndState::BtcRedeemed => AliceState::BtcRedeemed,
                AliceEndState::XmrRefunded => AliceState::XmrRefunded,
                AliceEndState::BtcPunished => AliceState::BtcPunished,
            },
        }
    }
}

impl Display for Alice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alice::WatchingForTxLockInMempool { .. } => write!(f, "Watching for TxLock in mempool"),
            Alice::WaitingForTxLockConfirmations { .. } => {
                write!(f, "Waiting for TxLock confirmations")
            }
            Alice::WaitingForEncSig { .. } => write!(f, "Waiting Bob to send EncSig"),
            Alice::CancelTimelockExpired { .. } => f.write_str("Cancel timelock is expired"),
            Alice::BtcCancelled { .. } => f.write_str("Bitcoin cancel transaction published"),
            Alice::BtcPunishable { .. } => f.write_str("Bitcoin punishable"),
            Alice::BtcRefunded { .. } => f.write_str("Monero refundable"),
            Alice::Done(end_state) => write!(f, "Done: {}", end_state),
            Alice::EncSigLearned { .. } => f.write_str("Encrypted signature learned"),
        }
    }
}
