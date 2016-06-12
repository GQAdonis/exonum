#[macro_use] mod spec;
#[cfg(test)] mod tests;

mod raw;
mod error;
mod fields;
mod protocol;

use super::crypto::{Hash, PublicKey};

pub use self::raw::{RawMessage, MessageBuffer, Message, HEADER_SIZE};
pub use self::error::{Error};
pub use self::fields::{Field, SegmentField};
pub use self::protocol::*;

// TODO: implement common methods for enum types (hash, raw, from_raw, verify)

pub enum Any {
    Basic(BasicMessage),
    Consensus(ConsensusMessage),
    Tx(TxMessage),
}

pub enum BasicMessage {
    Connect(Connect),
}

pub enum ConsensusMessage {
    Propose(Propose),
    Prevote(Prevote),
    Precommit(Precommit),
    Commit(Commit),
}

pub enum TxMessage {
    Issue(TxIssue),
    Transfer(TxTransfer),
    VoteValidator(TxVoteValidator),
    VoteConfig(TxVoteConfig),
}

impl TxMessage {
    pub fn hash(&self) -> Hash {
        match *self {
            TxMessage::Issue(ref msg) => msg.hash(),
            TxMessage::Transfer(ref msg) => msg.hash(),
            TxMessage::VoteValidator(ref msg) => msg.hash(),
            TxMessage::VoteConfig(ref msg) => msg.hash()
        }
    }
}

impl ConsensusMessage {
    pub fn validator(&self) -> u32 {
        match *self {
            ConsensusMessage::Propose(ref msg) => msg.validator(),
            ConsensusMessage::Prevote(ref msg) => msg.validator(),
            ConsensusMessage::Precommit(ref msg) => msg.validator(),
            ConsensusMessage::Commit(ref msg) => msg.validator(),
        }
    }

    pub fn height(&self) -> u64 {
        match *self {
            ConsensusMessage::Propose(ref msg) => msg.height(),
            ConsensusMessage::Prevote(ref msg) => msg.height(),
            ConsensusMessage::Precommit(ref msg) => msg.height(),
            ConsensusMessage::Commit(ref msg) => msg.height(),
        }
    }

    pub fn round(&self) -> u32 {
        match *self {
            ConsensusMessage::Propose(ref msg) => msg.round(),
            ConsensusMessage::Prevote(ref msg) => msg.round(),
            ConsensusMessage::Precommit(ref msg) => msg.round(),
            ConsensusMessage::Commit(ref msg) => msg.round(),
        }
    }

    pub fn raw(&self) -> &RawMessage {
        match *self {
            ConsensusMessage::Propose(ref msg) => msg.raw(),
            ConsensusMessage::Prevote(ref msg) => msg.raw(),
            ConsensusMessage::Precommit(ref msg) => msg.raw(),
            ConsensusMessage::Commit(ref msg) => msg.raw(),
        }
    }

    pub fn verify(&self, public_key: &PublicKey) -> bool {
        match *self {
            ConsensusMessage::Propose(ref msg) => msg.verify(public_key),
            ConsensusMessage::Prevote(ref msg) => msg.verify(public_key),
            ConsensusMessage::Precommit(ref msg) => msg.verify(public_key),
            ConsensusMessage::Commit(ref msg) => msg.verify(public_key),
        }
    }
}

impl Any {
    pub fn from_raw(raw: RawMessage) -> Result<Any, Error> {
        // TODO: check input message size
        Ok(match raw.message_type() {
            Connect::MESSAGE_TYPE => Any::Basic(BasicMessage::Connect(Connect::from_raw(raw)?)),
            Propose::MESSAGE_TYPE => Any::Consensus(ConsensusMessage::Propose(Propose::from_raw(raw)?)),
            Prevote::MESSAGE_TYPE => Any::Consensus(ConsensusMessage::Prevote(Prevote::from_raw(raw)?)),
            Precommit::MESSAGE_TYPE => Any::Consensus(ConsensusMessage::Precommit(Precommit::from_raw(raw)?)),
            Commit::MESSAGE_TYPE => Any::Consensus(ConsensusMessage::Commit(Commit::from_raw(raw)?)),
            _ => {
                // TODO: use result here
                panic!("unrecognized message type");
            }
        })
    }
}
