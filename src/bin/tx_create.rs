extern crate cardano;
extern crate chain_addr;
extern crate chain_core;
extern crate chain_crypto;
extern crate chain_impl_mockchain;
extern crate reqwest;
extern crate serde_json;
extern crate structopt;

mod utils;

use cardano::util::hex;
use chain_addr::{Address, Discrimination, Kind};
use chain_core::property::Serialize;
use chain_crypto::{Ed25519Extended, PublicKey};
use chain_impl_mockchain::fee::LinearFee;
use chain_impl_mockchain::key::SpendingSecretKey;
use chain_impl_mockchain::transaction::{Input, TransactionId, UtxoPointer};
use chain_impl_mockchain::txbuilder::{GeneratedTransaction, OutputPolicy, TransactionBuilder};
use chain_impl_mockchain::value::Value;
use std::io;
use structopt::StructOpt;
use utils::SegmentParser;

fn main() {
    TxCreate::from_args().exec();
}

/// Create transaction binary blob
#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct TxCreate {
    /// transaction input. Must have format `<hex-encoded-transaction-id>:<output-index>:<value>`.
    /// E.g. `1234567890abcdef:2:535`. At least 1 value required.
    #[structopt(name = "input", short, long, parse(try_from_str = "parse_input"))]
    inputs: Vec<Input>,
    /// transaction output. Must have format `<hex-encoded-address>:<value>`.
    /// E.g. `abcdef1234567890:501`. At least 1 value required.
    #[structopt(name = "output", short, long, parse(try_from_str = "parse_output"))]
    outputs: Vec<(Address, Value)>,
    /// return address. Value taken from inputs and not spent on outputs or fees will be returned
    /// to this address. If not provided, the excess will go to treasury. Must be hex-encoded.
    #[structopt(name = "return", short, long, parse(try_from_str = "parse_address"))]
    ret_addr: Option<Address>,
    /// fee base which will be always added to the transaction
    #[structopt(short = "f")]
    fee_base: Option<u64>,
    /// fee which will be added to the transaction for every input and output
    #[structopt(short = "a")]
    fee_per_addr: Option<u64>,
    /// fee which will be added to the transaction for certificate
    #[structopt(short = "c")]
    fee_per_cert: Option<u64>,
    /// transaction spending keys. Must be hex-encoded. Required as many as provided inputs.
    #[structopt(
        name = "spending_key",
        short,
        long,
        parse(try_from_str = "parse_spending_key")
    )]
    spending_keys: Vec<SpendingSecretKey>,
}

impl TxCreate {
    pub fn exec(self) {
        let mut builder = TransactionBuilder::new();
        for input in &self.inputs {
            builder.add_input(input);
        }
        for (address, value) in self.outputs {
            builder.add_output(address, value);
        }
        let fee = LinearFee::new(
            self.fee_base.unwrap_or(0),
            self.fee_per_addr.unwrap_or(0),
            self.fee_per_cert.unwrap_or(0),
        );
        let output_policy = match self.ret_addr {
            Some(addr) => OutputPolicy::One(addr),
            None => OutputPolicy::Forget,
        };
        let (_, mut finalizer) = builder.finalize(fee, output_policy).unwrap();
        for spending_key in &self.spending_keys {
            finalizer.sign(spending_key);
        }
        let output = io::stdout();
        match finalizer.build() {
            GeneratedTransaction::Type1(transaction) => transaction.serialize(output),
            GeneratedTransaction::Type2(transaction) => transaction.serialize(output),
        }
        .unwrap();
    }
}

fn parse_input(input: &str) -> Result<Input, String> {
    let mut parser = SegmentParser::new(input);
    let tx_id: TransactionId = parser.parse_next()?;
    let tx_idx: u8 = parser.parse_next()?;
    let value: u64 = parser.parse_next()?;
    parser.finish()?;
    let utxo_pointer = UtxoPointer::new(tx_id, tx_idx, Value(value));
    Ok(Input::from_utxo(utxo_pointer))
}

fn parse_output(input: &str) -> Result<(Address, Value), String> {
    let mut parser = SegmentParser::new(input);
    let addr_str = parser.get_next()?;
    let addr = parse_address(addr_str)?;
    let value: u64 = parser.parse_next()?;
    parser.finish()?;
    Ok((addr, Value(value)))
}

fn parse_address(input: &str) -> Result<Address, String> {
    let addr_bytes =
        hex::decode(input).map_err(|e| format!("failed to parse address hex: {}", e))?;
    let addr_key = PublicKey::<Ed25519Extended>::from_bytes(&addr_bytes)
        .map_err(|e| format!("failed to parse address: {}", e))?;
    Ok(Address(Discrimination::Test, Kind::Single(addr_key)))
}

fn parse_spending_key(input: &str) -> Result<SpendingSecretKey, String> {
    let addr_bytes =
        hex::decode(input).map_err(|e| format!("failed to parse spending key hex: {}", e))?;
    SpendingSecretKey::from_bytes(&addr_bytes)
        .map_err(|e| format!("failed to parse spending key: {}", e))
}
