use serpentine::utils::*;
use std::fmt;


#[derive(Debug, Clone)]
pub struct Block {
	id: u16,
	prev: u16,
	entries: Vec<Transaction>,
}

pub fn new_block(entries_: &[Transaction], prev: u16) -> Block {
	let id = prev + 1;
	let entries = entries_.to_vec();

	Block { id, prev, entries }
}

impl fmt::Display for Block {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"<Block id={:04x?} size={} />",
			self.id, self.entries.len()
		)
	}
}
