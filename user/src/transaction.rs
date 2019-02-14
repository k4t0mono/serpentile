use std::fmt;
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpStream; 
use byteorder::*;


#[derive(Debug)]
pub struct Transaction {
	from: u16,
	to: u16,
	value: f32,
}

impl Transaction {
	pub fn new(from: u16, to: u16, value: f32) -> Transaction {
		let t = Transaction{ from, to, value };
		info!("New trasanction: {}", t);

		t
	}

	pub fn broadcast(&self) -> Result<()> {
		let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();	
		stream.write(&self.serialize()).unwrap();	

		Ok(())
	}

	fn serialize(&self) -> [u8; 9] {
		let mut buf: [u8; 9] = [0; 9];

		BigEndian::write_u16(&mut buf[0 .. 2], self.from);
		BigEndian::write_u16(&mut buf[2 .. 4], self.to);
		BigEndian::write_f32(&mut buf[4 .. 8], self.value);

		let mut cs = 0x00;
		for b in buf.iter() { cs ^= b }
		buf[8] = cs;

		buf
	}
}

impl fmt::Display for Transaction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<Transaction from={:02X?} to={:02X?} value={} />", self.from, self.to, self.value)
	}
}
