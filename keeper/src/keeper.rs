use serpentine::*;
use std::net::SocketAddr;


#[derive(Debug)]
pub struct Keeper {
	transactions: Vec<Transaction>,
	blocks: Vec<Block>,
	block_listeners: Vec<SocketAddr>,
	transaction_listeners: Vec<SocketAddr>,
}

impl Keeper {
	pub fn new() -> Keeper {
		let transactions: Vec<Transaction> = Vec::new();
		let blocks: Vec<Block> = Vec::new();
		let block_listeners: Vec<SocketAddr> = Vec::new();
		let transaction_listeners: Vec<SocketAddr> = Vec::new();

		Keeper{ transactions, blocks, block_listeners, transaction_listeners }
	}


	pub fn add_transaction(&mut self, t: Transaction) {
		info!("Recived: {}", t);

		self.transactions.push(t);
		self.inspect();
	}

	pub fn add_block(&mut self, b: Block) {
		info!("Recived: {}", b);

		self.blocks.push(b);
		self.inspect();
	}

	pub fn add_block_listener(&mut self, sa: SocketAddr) {
		info!("New block listener: {}", sa);

		self.block_listeners.push(sa);
		self.inspect();
	}

	pub fn add_transaction_listener(&mut self, sa: SocketAddr) {
		info!("New transaction listener: {}", sa);

		self.transaction_listeners.push(sa);
		self.inspect();
	}

	pub fn add_block_transaction_listener(&mut self, sa: SocketAddr) {
		info!("New block transaction listener: {}", sa);

		self.block_listeners.push(sa);
		self.transaction_listeners.push(sa);
		self.inspect();
	}

	pub fn inspect(&self) {
		trace!("{:#?}", self);
	}
}
