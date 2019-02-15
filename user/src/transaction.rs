use std::fmt;
use byteorder::*;
use crc::crc16;


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

	pub fn serialize(&self) -> [u8; 10] {
		let mut buf: [u8; 10] = [0; 10];

		BigEndian::write_u16(&mut buf[0 .. 2], self.from);
		BigEndian::write_u16(&mut buf[2 .. 4], self.to);
		BigEndian::write_f32(&mut buf[4 .. 8], self.value);

		let cs = crc16::checksum_usb(&buf);
		BigEndian::write_u16(&mut buf[8 .. 10], cs);

		buf
	}
}

impl fmt::Display for Transaction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<Transaction from={:04X?} to={:04X?} value={:.2} />", self.from, self.to, self.value)
	}
}
