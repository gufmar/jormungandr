//! This module provides the different abstractions for the different
//! part of the blockchain.
//!
//! It has been split into 3 components:
//!
//! 1. chain: all the components that chains blocks together;
//! 2. ledger: the transaction model of a blockchain;
//! 3. consensus: the consensus model of the blockchain.
//!

pub mod chain;
pub mod ledger;
// TODO: pub mod consensus;

pub trait BlockConfig {
    type Block: chain::Block<Hash = Self::BlockHash>
              + ledger::HasTransaction<Transaction = Self::Transaction>;
    type BlockHash;
    type BlockHeader;
    type Transaction: ledger::Transaction<Id = Self::TransactionId>;
    type TransactionId;
    type GenesisData;

    type Ledger: ledger::Ledger<Transaction = Self::Transaction>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cardano;
impl BlockConfig for Cardano {
    type Block = chain::cardano::Block;
    type BlockHash = chain::cardano::BlockHash;
    type BlockHeader = chain::cardano::Header;
    type Transaction = chain::cardano::Transaction;
    type TransactionId = chain::cardano::TransactionId;
    type GenesisData = chain::cardano::GenesisData;
    type Ledger = cardano::block::verify_chain::ChainState;
}