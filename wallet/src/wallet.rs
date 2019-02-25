use std::fmt;
use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};
use serpentine::Transaction;


#[derive(Debug)]
pub struct Wallet {
    id: u16,
    addrs: Vec<SocketAddr>,
}


impl Wallet {
    pub fn new(id: u16, addrs: Vec<SocketAddr>) -> Wallet {
        let u = Wallet{ id, addrs };
        info!("New user: {}", u);

        u
    }

    pub fn new_transaction(&self, to: u16, value: f32) -> Result<Transaction, &'static str> {
        let t = Transaction::new(self.id, to, value);

        let mut stream = match TcpStream::connect(&self.addrs[..]) {
           Ok(s) => s,
           Err(_) => return Err("Could not broadcast"),
        };

        let mut buf = t.serialize().unwrap();
        buf.insert(0, 0x10);
        stream.write(&buf).unwrap();

        Ok(t)
    }
}


impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Wallet id={:02X?} />", self.id)
    }
}
