#[macro_use] extern crate log;
extern crate byteorder;
extern crate crc;


pub mod utils {
	use byteorder::*;
	use crc::crc16;
	use std::fmt;
	use std::io::{Cursor, Error, ErrorKind, Result};

	#[derive(Debug)]
	pub struct Transaction {
		from: u16,
		to: u16,
		value: f32,
	}

	impl Transaction {
		pub fn new(from: u16, to: u16, value: f32) -> Transaction {
			Transaction{ from, to, value }
		}

		pub fn serialize(&self) -> [u8; 10] {
			let mut buf: [u8; 10] = [0; 10];

			BigEndian::write_u16(&mut buf[0..2], self.from);
			BigEndian::write_u16(&mut buf[2..4], self.to);
			BigEndian::write_f32(&mut buf[4..8], self.value);

			let cs = crc16::checksum_usb(&buf[0..8]);
			BigEndian::write_u16(&mut buf[8..10], cs);

			buf
		}

		pub fn deserialize(buf: [u8; 10]) -> Result<Transaction> {
			let cs_r = Cursor::new(&buf[8..10]).read_u16::<BigEndian>()?;
			let cs_c = crc16::checksum_usb(&buf[0..8]);
			if cs_r != cs_c  { return Err(Error::new(ErrorKind::InvalidData, "Invalid CRC")) }

			let from = Cursor::new(&buf[0..2]).read_u16::<BigEndian>()?;
			let to = Cursor::new(&buf[2..4]).read_u16::<BigEndian>()?;
			let value = Cursor::new(&buf[4..8]).read_f32::<BigEndian>()?;

			Ok(Transaction{ from, to, value })
		}
	}

	impl fmt::Display for Transaction {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(
				f,
				"<Transaction from={:04X?} to={:04X?} value={:.2} />",
				self.from, self.to, self.value
			)
		}
	}
}
