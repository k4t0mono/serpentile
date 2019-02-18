use serpentine::*;


#[derive(Debug)]
pub struct Keeper {
	transactions: Vec<Transaction>,
}

impl Keeper {
	pub fn new() -> Keeper {
		let transactions: Vec<Transaction> = Vec::new();

		Keeper{ transactions }
	}


	pub fn add_transaction(&mut self, t: Transaction) {
		self.transactions.push(t);
	}
}
