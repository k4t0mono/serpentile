use crc::crc32;
use std::boxed::Box;
use std::fmt;
use bincode::{serialize, deserialize, ErrorKind};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
	id: String,
	from: u16,
	to: u16,
	value: f32,
}

impl Transaction {
	pub fn new(from: u16, to: u16, value: f32) -> Transaction {
		let id = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(16)
			.collect();

		Transaction{ id, from, to, value }
	}

	pub fn serialize(&self) -> bincode::Result<Vec<u8>> {
		let mut encoded: Vec<u8> = serialize(&self)?;

		let checksum = crc32::checksum_ieee(&encoded[..]);
		let mut cs = serialize(&checksum)?;

		cs.append(&mut encoded);

		Ok(cs)
	}

	pub fn deserialize(encoded: &[u8]) -> bincode::Result<Transaction> {
		let cs_r: u32 = deserialize(&encoded[..4])?;
		let cs_c = crc32::checksum_ieee(&encoded[4..]);
		if cs_c != cs_r {
			return Err(Box::new(ErrorKind::Custom("Invalid CRC".to_string())));
		}
		
		let t: Transaction = deserialize(&encoded[4..])?;

		Ok(t)
	}
}

impl fmt::Display for Transaction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"<Transaction id={} from={:04X?} to={:04X?} value={:.2} />",
			self.id, self.from, self.to, self.value
		)
	}
}
