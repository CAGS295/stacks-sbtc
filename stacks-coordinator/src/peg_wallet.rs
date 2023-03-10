use bitcoin::consensus::Decodable;
use serde::Serialize;

use crate::bitcoin_node;
use crate::bitcoin_node::BitcoinTransaction;
use crate::error::Result;
use crate::stacks_node;
use crate::stacks_node::{PegInOp, PegOutRequestOp};
use crate::stacks_transaction::StacksTransaction;

pub trait StacksWallet {
    fn mint(&mut self, op: &stacks_node::PegInOp) -> Result<StacksTransaction>;
    fn burn(&mut self, op: &stacks_node::PegOutRequestOp) -> Result<StacksTransaction>;
    fn set_wallet_address(&mut self, address: PegWalletAddress) -> Result<StacksTransaction>;
}

pub trait BitcoinWallet {
    fn fulfill_peg_out(
        &self,
        op: &stacks_node::PegOutRequestOp,
    ) -> bitcoin_node::BitcoinTransaction;
}

pub trait PegWallet {
    type StacksWallet: StacksWallet;
    type BitcoinWallet: BitcoinWallet;
    fn stacks_mut(&mut self) -> &mut Self::StacksWallet;
    fn bitcoin_mut(&mut self) -> &mut Self::BitcoinWallet;
}

// TODO: Representation
// Should correspond to a [u8; 32] - perhaps reuse a FROST type?
#[derive(Serialize)]
pub struct PegWalletAddress(pub [u8; 32]);

pub struct WrapPegWallet {}

impl PegWallet for WrapPegWallet {
    type StacksWallet = FileStacksWallet;
    type BitcoinWallet = FileBitcoinWallet;

    fn stacks_mut(&mut self) -> &mut Self::StacksWallet {
        todo!()
    }

    fn bitcoin_mut(&mut self) -> &mut Self::BitcoinWallet {
        todo!()
    }
}

pub struct FileStacksWallet {}

impl StacksWallet for FileStacksWallet {
    fn mint(&mut self, op: &PegInOp) -> Result<StacksTransaction> {
        todo!()
    }

    fn burn(&mut self, op: &PegOutRequestOp) -> Result<StacksTransaction> {
        todo!()
    }

    fn set_wallet_address(&mut self, address: PegWalletAddress) -> Result<StacksTransaction> {
        todo!()
    }
}

pub struct FileBitcoinWallet {}

impl BitcoinWallet for FileBitcoinWallet {
    fn fulfill_peg_out(&self, op: &PegOutRequestOp) -> BitcoinTransaction {
        BitcoinTransaction::consensus_decode(&mut "".as_bytes()).unwrap()
    }
}
