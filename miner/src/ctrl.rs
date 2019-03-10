use serpentine::{Block, Transaction};
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Ctrl {
    max_size: usize,
    entries: Vec<Transaction>,
    prev_block: String,
    blocks: Vec<Block>,
}

impl Ctrl {
    pub fn new(max_size: usize) -> Ctrl {
        let entries = Vec::new();
        let blocks = Vec::new();
        let prev_block = "".to_string();

        Ctrl{ max_size, entries, blocks, prev_block }
    }

    pub fn add_entry(&mut self, t: Transaction) {
        self.entries.push(t);

        if self.entries.len() == self.max_size { self.create_block(); }
    }

    fn create_block(&mut self) {
        debug!("Creating block");
        let b = Block::new(&self.entries[0..5], self.prev_block.clone());
        info!("Created: {}", b);

        self.prev_block = b.get_id();
        self.blocks.push(b);
        self.entries.clear();
        self.transmit_last_block();
    }

    fn transmit_last_block(&self) {
        let mut stream = TcpStream::connect("127.0.0.1:58913").unwrap();

        let block = &self.blocks[self.blocks.len() - 1];
        let mut buff = vec![0x20];
        buff.append(&mut block.serialize().unwrap());

        stream.write(&buff).unwrap();
    }
}
