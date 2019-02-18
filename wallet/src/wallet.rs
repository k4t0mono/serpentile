use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream; 
use serpentine::Transaction;


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

		let mut stream = TcpStream::connect("127.0.0.1:58913").unwrap();

		let mut buf = t.serialize().unwrap();
		buf.insert(0, 0x10);

		stream.write(&buf).unwrap();	
		info!("Sended: {}", t);
	}
}


impl fmt::Display for Wallet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<Wallet id={:02X?} />", self.id)
	}
}
