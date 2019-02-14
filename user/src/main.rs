#[macro_use] extern crate log;
extern crate simplelog;
extern crate byteorder;

mod transaction;
mod user;

use rand::Rng;
use user::*;


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
	info!("Starting user.rs");

	let id: u16 = rand::thread_rng().gen();
	let user = User::new(id);

	user.new_transaction(0x1234, 20.0);
}
