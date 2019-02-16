#[macro_use] extern crate serde_derive;
extern crate bincode;
extern crate crc;


pub mod utils {
	use crc::crc16;
	use std::boxed::Box;
	use std::fmt;
	use bincode::{serialize, deserialize, ErrorKind};

	#[derive(Serialize, Deserialize, Debug, Clone)]
	pub struct Transaction {
		from: u16,
		to: u16,
		value: f32,
	}

	impl Transaction {
		pub fn new(from: u16, to: u16, value: f32) -> Transaction {
			Transaction{ from, to, value }
		}

		pub fn serialize(&self) -> bincode::Result<Vec<u8>> {
			let mut encoded: Vec<u8> = serialize(&self)?;

			let checksum = crc16::checksum_usb(&encoded[..]);
			let mut cs = serialize(&checksum)?;

			cs.append(&mut encoded);

			Ok(cs)
		}

		pub fn deserialize(encoded: &[u8]) -> bincode::Result<Transaction> {
			let t: Transaction = deserialize(&encoded[2..])?;

			let cs_r: u16 = deserialize(&encoded[..2])?;
			let cs_c = crc16::checksum_usb(&encoded[2..]);
			if cs_c != cs_r {
				return Err(Box::new(ErrorKind::Custom("Invalid CRC".to_string())));
			}
			
			Ok(t)
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
