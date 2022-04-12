use bit_vec::BitVec;
use im::HashSet;
use primitive_types::U256;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

use crate::constants::*;

pub type U256Map<T> = HashMap<U256, T>;

pub type Hash = U256;

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
  pub value: [u8; BODY_SIZE],
}

#[derive(Debug, Clone)]
pub struct Block {
  pub prev: U256, // previous block (32 bytes)
  pub time: u64,  // block timestamp
  pub rand: u64,  // block nonce
  pub body: Body, // block contents (1280 bytes)
}

pub type Transaction = Vec<u8>;

pub struct NodeState {
  pub tip      : (U256, U256),                   // current tip work and block hash
  pub block    : U256Map<Block>,                 // block hash -> block information
  pub children : U256Map<Vec<U256>>,             // block hash -> blocks that have this as its parent
  pub waiters  : U256Map<Vec<Block>>,            // block hash -> blocks that are waiting for this block info
  pub work     : U256Map<U256>,                  // block hash -> accumulated work
  pub target   : U256Map<U256>,                  // block hash -> this block's target
  pub height   : U256Map<u64>,                   // block hash -> cached height
  pub seen     : U256Map<bool>,                  // block hash -> have we received it yet?
  pub mined    : U256Map<HashSet<Transaction>>,  // block hash -> set of transaction hashes that were already mined
  pub pool     : PriorityQueue<Transaction,u64>, // transactions to be mined
}

pub enum Address {
  IPv4 {
    val0: u8,
    val1: u8,
    val2: u8,
    val3: u8,
    port: u16,
  }
}

pub struct Peer {
  pub seen_at: u64,
  pub address: Address,
}

pub enum Message {
  PutPeers {
    peers: Vec<Address>
  },
  PutBlock {
    block: Block
  },
  AskBlock {
    bhash: Hash
  }
}
