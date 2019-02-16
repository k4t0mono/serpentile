use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream; 
use serpentine::utils::*;


#[derive(Debug)]
pub struct Wallet {
	id: u16
}


impl Wallet {
	pub fn new(id: u16) -> Wallet {
		let u = Wallet{ id };
		info!("New user: {}", u);

		u
	}

	pub fn new_transaction(&self, to: u16, value: f32) {
		let t = Transaction::new(self.id, to, value);

		let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();	
		stream.write(&t.serialize()).unwrap();	
		info!("Sended: {}", t);
	}
}


impl fmt::Display for Wallet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<Wallet id={:02X?} />", self.id)
	}
}
