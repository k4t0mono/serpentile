use crate::transaction::*;
use std::fmt;
use sha3::{Sha3_256, Digest};
use bincode::serialize;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[derive(Debug, Clone, Serialize)]
pub struct Block {
	id: String,
	prev: String,
	entries: Vec<Transaction>,
	magic: u32,
	hash: String,
}

impl Block {
	pub fn new(entries_: &[Transaction], prev: String) -> Block {
		let id = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(8)
			.collect();

		let entries = entries_.to_vec();
		let magic = 0;
		let hash = "".to_string();

		let bo = Block { id, prev, entries, magic, hash };
		let b = calc_magic(bo).unwrap();

		b
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}
}

fn calc_magic(mut block: Block) -> bincode::Result<Block> {
	loop {
		let mut hasher = Sha3_256::new();
		hasher.input(serialize(&block)?);
		let hash = hasher.result();

		if (hash[0] == 0xe6) && (hash[1] == 0x21) {
			block.hash = base64::encode_config(&hash, base64::URL_SAFE);
			break;
		}

		block.magic = random();
	}

	Ok(block)
}

impl fmt::Display for Block {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"<Block id={} size={} magic={:08x?} hash={} />",
			self.id, self.entries.len(), self.magic, self.hash
		)
	}
}
