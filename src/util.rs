use bit_vec::BitVec;
use im::HashSet;
use primitive_types::U256;
use priority_queue::PriorityQueue;
use sha3::Digest;
use std::collections::HashMap;
use std::net::*;

use crate::bits::*;

pub type U256Map<T> = HashMap<U256, T>;
pub type Hash = U256;

// Logs
// ====

#[macro_export]
macro_rules! dbg_println {
    () => {
        #[cfg(debug_assertions)]
        std::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        // $crate::io::_eprint($crate::format_args_nl!($($arg)*));
        eprintln!($($arg)*);
    }};
}

// Numerics
// ========

// Size of a u128, in bytes
pub const U128_SIZE : usize = 128 / 8;

pub fn u256(x: u128) -> U256 {
  return U256::from(x);
}

pub fn next_power_of_two(x: f64) -> f64 {
  if x <= 1.0 { x } else { (2.0_f64).powf(x.log2().floor() + 1.0) }
}

pub fn u64_to_bytes(value: u64) -> Vec<u8> {
  return Vec::from(value.to_le_bytes());
}

pub fn u128_to_bytes(value: u128) -> Vec<u8> {
  return Vec::from(value.to_le_bytes());
}

pub fn u256_to_bytes(value: U256) -> Vec<u8> {
  // TODO: primitive_types::U256::to_big_endian ?
  let mut bytes = Vec::new();
  for i in 0 .. 32 {
    bytes.push(value.byte(32 - i - 1));
  }
  return bytes;
}

pub fn u256_to_hex(value: U256) -> String {
  hex::encode(u256_to_bytes(value))
}

pub fn bitvec_to_bytes(bits: &BitVec) -> Vec<u8> {
  return bits.to_bytes();
}

pub fn bytes_to_bitvec(bytes: &[u8]) -> BitVec {
  return BitVec::from_bytes(bytes);
}

pub fn u128s_to_u8s(u128s: &[u128]) -> Vec<u8> {
  let mut u8s : Vec<u8> = vec![];
  for i in 0 .. u128s.len() {
    u8s.extend_from_slice(&mut u128s[i].to_le_bytes());
  }
  return u8s;
}

pub fn u8s_to_u128s(u8s: &[u8]) -> Vec<u128> {
  let mut u8s = u8s.to_vec();
  u8s.resize((u8s.len() + 15) / 16 * 16, 0);
  let mut u128s : Vec<u128> = vec![];
  for i in 0 .. u8s.len() / 16 {
    u128s.push(u128::from_le_bytes(u8s[i * 16 .. i * 16 + 16].try_into().unwrap()));
  }
  return u128s;
}

// System
// ======

// Gets current timestamp in milliseconds
pub fn get_time() -> u128 {
  return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u128;
}
