use std::fmt;
use crate::transaction::*;


#[derive(Debug)]
pub struct User {
	id: u16
}


impl User {
	pub fn new(id: u16) -> User {
		let u = User{ id };
		info!("New user: {}", u);

		u
	}

	pub fn new_transaction(&self, to: u16, value: f32) {
		let t = Transaction::new(self.id, to, value);
		
		let _res = t.broadcast().unwrap();
	}
}


impl fmt::Display for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<User id={:02X?} />", self.id)
	}
}
