use serpentine::transaction::*;
use serpentine::block::*;

pub struct Ctrl {
	max_size: usize,
	entries: Vec<Transaction>,
	prev_block: u16,
	blocks: Vec<Block>,
}

impl Ctrl {
	pub fn new(max_size: usize) -> Ctrl {
		let entries = Vec::new();
		let blocks = Vec::new();
		let prev_block = 0x00;

		Ctrl{ max_size, entries, blocks, prev_block }
	}

	pub fn add_entry(&mut self, t: Transaction) {
		self.entries.push(t);

		if self.entries.len() == self.max_size { self.create_block(); }
	}

	fn create_block(&mut self) {
		let b = new_block(&self.entries[0..5], self.prev_block);
		info!("Created: {}", b);

		self.blocks.push(b);
		self.entries.clear();
		self.prev_block += 1;
	}
}
