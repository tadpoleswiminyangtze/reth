use core::fmt;

use alloy_eips::eip2718::Eip2718Error;
use alloy_primitives::{U64, U8};
use alloy_rlp::{Decodable, Encodable};
use reth_codecs::Compact;

/// Helper trait that unifies all behaviour required by transaction type ID to support full node
/// operations.
pub trait FullTxType: TxType + Compact {}

impl<T> FullTxType for T where T: TxType + Compact {}

/// Trait representing the behavior of a transaction type.
pub trait TxType:
    Send
    + Sync
    + Unpin
    + Clone
    + Copy
    + Default
    + fmt::Debug
    + fmt::Display
    + PartialEq
    + Eq
    + PartialEq<u8>
    + Into<u8>
    + Into<U8>
    + TryFrom<u8, Error = Eip2718Error>
    + TryFrom<u64>
    + TryFrom<U64>
    + Encodable
    + Decodable
{
}

impl<T> TxType for T where
    T: Send
        + Sync
        + Unpin
        + Clone
        + Copy
        + Default
        + fmt::Debug
        + fmt::Display
        + PartialEq
        + Eq
        + PartialEq<u8>
        + Into<u8>
        + Into<U8>
        + TryFrom<u8, Error = Eip2718Error>
        + TryFrom<u64>
        + TryFrom<U64>
        + Encodable
        + Decodable
{
}
