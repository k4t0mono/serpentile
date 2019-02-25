use crate::transaction::*;
use std::fmt;
use sha3::{Sha3_256, Digest};
use bincode::ErrorKind;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crc::crc32;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    id: String,
    prev: String,
    entries: Vec<Transaction>,
    magic: u32,
    hash: String,
}

impl Block {
    pub fn new(entries_: &[Transaction], prev: String) -> Block {
        let id = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .collect();

        let entries = entries_.to_vec();
        let magic = 0;
        let hash = "".to_string();

        let bo = Block { id, prev, entries, magic, hash };
        let b = calc_magic(bo).unwrap();

        b
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn serialize(&self) -> bincode::Result<Vec<u8>> {
        let mut encoded = bincode::serialize(&self)?;

        let checksum = crc32::checksum_ieee(&encoded[..]);
        let mut cs = bincode::serialize(&checksum)?;

        cs.append(&mut encoded);

        Ok(cs)
    }

    pub fn deserialize(encoded: &[u8]) -> bincode::Result<Block> {
        let cs_r: u32 = bincode::deserialize(&encoded[..4])?;
        let cs_c = crc32::checksum_ieee(&encoded[4..]);
        if cs_c != cs_r {
            return Err(Box::new(ErrorKind::Custom("Invalid CRC".to_string())));
        }

        let b: Block = bincode::deserialize(&encoded[4..])?;
        
        Ok(b)
    }
}

fn calc_magic(mut block: Block) -> bincode::Result<Block> {
    loop {
        let mut hasher = Sha3_256::new();
        hasher.input(bincode::serialize(&block)?);
        let hash = hasher.result();

        if (hash[0] == 0xe6) && (hash[1] == 0x21) {
            block.hash = base64::encode_config(&hash, base64::URL_SAFE);
            break;
        }

        block.magic = random();
    }

    Ok(block)
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<Block id={} size={} magic={:08x?} hash={} />",
            self.id, self.entries.len(), self.magic, self.hash
        )
    }
}
