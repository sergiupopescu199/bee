// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::{Balance, Error as TypesError, Unspent},
    workers::snapshot::error::Error as SnapshotError,
};

use bee_message::{address::Address, Error as MessageError, MessageId};

/// Errors occurring during consensus.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    Snapshot(#[from] SnapshotError),
    #[error("")]
    Types(#[from] TypesError),
    #[error("Message error: {0}")]
    Message(#[from] MessageError),
    #[error("Message {0} is missing in the past cone of the milestone")]
    MissingMessage(MessageId),
    #[error("")]
    UnsupportedInputKind(u8),
    #[error("")]
    UnsupportedOutputKind(u8),
    #[error("")]
    UnsupportedAddressKind(u8),
    #[error("")]
    UnsupportedTransactionEssenceKind(u8),
    #[error("")]
    UnsupportedPayloadKind(u32),
    #[error("Message was not found")]
    MilestoneMessageNotFound,
    #[error("Message payload was not a milestone")]
    NoMilestonePayload,
    #[error("Tried to confirm {0} on top of {1}")]
    NonContiguousMilestone(u32, u32),
    #[error("The computed merkle proof on milestone {0} does not match the one provided by the coordinator {1}")]
    MerkleProofMismatch(String, String),
    #[error("Invalid messages count: referenced ({0}) != no transaction ({1}) + conflicting ({2}) + included ({3})")]
    InvalidMessagesCount(usize, usize, usize, usize),
    #[error("Invalid ledger unspent state: {0}")]
    InvalidLedgerUnspentState(u64),
    #[error("Invalid ledger balance state: {0}")]
    InvalidLedgerBalanceState(u64),
    #[error("Invalid ledger dust state: {0:?} {1:?}")]
    InvalidLedgerDustState(Address, Balance),
    #[error("Consumed amount overflow: {0}.")]
    ConsumedAmountOverflow(u128),
    #[error("Created amount overflow: {0}.")]
    CreatedAmountOverflow(u128),
    #[error("Ledger state overflow: {0}")]
    LedgerStateOverflow(u128),
    #[error("Non zero balance diff sum: {0}.")]
    NonZeroBalanceDiffSum(i64),
    #[error("Decreasing receipt migrated at index")]
    DecreasingReceiptMigratedAtIndex,
    #[error("Missing unspent output: {0}")]
    MissingUnspentOutput(Unspent),
    #[error("")]
    Storage(Box<dyn std::error::Error + Send>),
}
