use serpentine::{Block, Transaction};

pub struct Ctrl {
	max_size: usize,
	entries: Vec<Transaction>,
	prev_block: String,
	blocks: Vec<Block>,
}

impl Ctrl {
	pub fn new(max_size: usize) -> Ctrl {
		let entries = Vec::new();
		let blocks = Vec::new();
		let prev_block = "".to_string();

		Ctrl{ max_size, entries, blocks, prev_block }
	}

	pub fn add_entry(&mut self, t: Transaction) {
		self.entries.push(t);

		if self.entries.len() == self.max_size { self.create_block(); }
	}

	fn create_block(&mut self) {
		debug!("Creating block");
		let b = Block::new(&self.entries[0..5], self.prev_block.clone());
		info!("Created: {}", b);

		self.prev_block = b.get_id();
		self.blocks.push(b);
		self.entries.clear();
	}
}
