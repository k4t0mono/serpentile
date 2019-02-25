#[macro_use] extern crate log;
extern crate simplelog;
extern crate byteorder;
extern crate crc;
extern crate serpentine;

mod config;
mod wallet;

use config::Config;
use rand::Rng;
use wallet::*;
use std::{thread, time, env, process};


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("Usage: wallet <num-messages> [hosts-file]");
        process::exit(1);
    });

    info!("Starting wallet.rs");

    let id: u16 = rand::thread_rng().gen();
    let wallet = Wallet::new(id, config.keeper_addrs);

    for i in 0..config.n_messages {
        match wallet.new_transaction(0x0032 + ((i as u16) << 8), 20.0 + (i as f32) / 10.0) {
            Ok(t) => info!("Broadcasted: {}", t),
            Err(e) => error!("{}", e),
        };

        thread::sleep(time::Duration::from_millis(750));
    }
}
