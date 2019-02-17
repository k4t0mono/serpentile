use crate::transaction::*;
use md5::{Md5, Digest};
use std::fmt;
use bincode::{serialize, deserialize, ErrorKind};


#[derive(Debug, Clone, Serialize)]
pub struct Block {
	id: u16,
	prev: u16,
	entries: Vec<Transaction>,
	magic: u32,
	hash: [u8; 16],
}

pub fn new_block(entries_: &[Transaction], prev: u16) -> Block {
	let id = prev + 1;
	let entries = entries_.to_vec();
	let magic = 0;
	let hash = [0; 16];

	let bo = Block { id, prev, entries, magic, hash };
	let b = calc_magic(bo).unwrap();

	b
}

pub fn calc_magic(mut block: Block) -> bincode::Result<Block> {
	let mut hr = Md5::new();
	hr.input(b":3");
	let mut hash = hr.result();

	loop {
		let mut hasher = Md5::new();
		hasher.input(serialize(&block)?);
		hash = hasher.result();

		if (hash[0] == 0xe6) && (hash[1] == 0x21) { break; }

		block.magic += 1;
	}
	for i in 0..16 { block.hash[i] = hash[i]; }

	Ok(block)
}

impl fmt::Display for Block {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s: String = "".to_string();
		for b in self.hash.iter() { s += &format!("{:02x?}", b); }

		write!(
			f,
			"<Block id={:04x?} size={} magic={:08x?} hash={} />",
			self.id, self.entries.len(), self.magic, s
		)
	}
}
