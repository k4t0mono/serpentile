#[macro_use] extern crate log;
extern crate simplelog;
extern crate byteorder;
extern crate crc;
extern crate serpentine;

mod wallet;

use rand::Rng;
use wallet::*;
use std::{thread, time};


fn set_logger(level: usize) {
	use simplelog::*;

	let log_level: LevelFilter = match level {
		0 => LevelFilter::Off,
		1 => LevelFilter::Error,
		2 => LevelFilter::Warn,
		3 => LevelFilter::Info,
		4 => LevelFilter::Debug,
		_ => LevelFilter::Trace,
	};

	TermLogger::init(log_level, Config::default()).unwrap();
}


fn main() {
	set_logger(3);
	info!("Starting wallet.rs");

	let id: u16 = rand::thread_rng().gen();
	let wallet = Wallet::new(id);

	for i in 0..1 {
		wallet.new_transaction(0xf032 + (i << 8), 20.0 + (i as f32) / 10.0);
		thread::sleep(time::Duration::from_millis(500));
	}
}
