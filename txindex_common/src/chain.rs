pub use bitcoin::network::Network as BNetwork;
use serde::Serialize;

pub use bitcoin::{
    address, blockdata::block::Header as BlockHeader, blockdata::script, consensus::deserialize,
    hash_types::TxMerkleNode, Address, Block, BlockHash, OutPoint, ScriptBuf as Script, Sequence,
    Transaction, TxIn, TxOut, Txid,
};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,
    Signet,
}


impl From<&String> for Network {
    fn from(network_name: &String) -> Self {
        let nn = network_name.as_str();
        match nn {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,
            "signet" => Network::Signet,
            _ => panic!("unsupported Bitcoin network: {:?}", network_name),
        }
    }
  }

impl From<&str> for Network {
  fn from(network_name: &str) -> Self {
      match network_name {
          "mainnet" => Network::Bitcoin,
          "testnet" => Network::Testnet,
          "regtest" => Network::Regtest,
          "signet" => Network::Signet,
          _ => panic!("unsupported Bitcoin network: {:?}", network_name),
      }
  }
}

impl From<Network> for BNetwork {
  fn from(network: Network) -> Self {
      match network {
          Network::Bitcoin => BNetwork::Bitcoin,
          Network::Testnet => BNetwork::Testnet,
          Network::Regtest => BNetwork::Regtest,
          Network::Signet => BNetwork::Signet,
      }
  }
}



#[cfg(not(feature = "liquid"))]
pub type Value = u64;


impl Network {
  pub fn magic(self) -> u32 {
      u32::from_le_bytes(BNetwork::from(self).magic().to_bytes())
  }

  pub fn is_regtest(self) -> bool {
      match self {
          Network::Regtest => true,
          _ => false,
      }
  }

  pub fn names() -> Vec<String> {
      return vec![
          "mainnet".to_string(),
          "testnet".to_string(),
          "regtest".to_string(),
          "signet".to_string(),
      ];
  }
}


pub fn genesis_hash(network: Network) -> BlockHash {
    bitcoin::blockdata::constants::genesis_block(BNetwork::from(network))
        .block_hash()
}
