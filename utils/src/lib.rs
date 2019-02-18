#[macro_use] extern crate serde_derive;
extern crate bincode;
extern crate rand;
extern crate crc;

mod transaction;
mod block;

pub use crate::block::Block;
pub use crate::transaction::Transaction;
