#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

mod ctrl;
mod block;

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::option::Option;
use serpentine::utils::*;
use ctrl::*;


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


fn parse_args() -> (usize) {
	let args: Vec<String> = std::env::args().collect();
	
	if args.len() < 1 {
		eprintln!("Usage: {} [log-level]", args[0]);
		panic!("Missing args");
	}

	let log_level = args.last().unwrap().parse::<usize>().unwrap_or(3);

	(log_level)
}

fn handle_client(mut stream: TcpStream) -> Option<Transaction> {
	trace!("New connection from {}", stream.peer_addr().unwrap());

	let mut buf: Vec<u8> = Vec::new();
	stream.read_to_end(&mut buf).unwrap();

	let t = match Transaction::deserialize(&buf[1..]) {
		Ok(v) => v,
		Err(e) => {
			debug!("Couldn't read a transaction. Error: {}", e);
			return None;
		},
	};
	debug!("Recived: {}", t);

	Some(t)
}

fn main() {
	let log_level = parse_args();
	set_logger(log_level);
	info!("Starting miner.rs");

	let mut ctrl = Ctrl::new(5);

	let listener = TcpListener::bind("0.0.0.0:34254").unwrap();
	for stream in listener.incoming() {
		let t = handle_client(stream.unwrap());
		if t.is_some() { ctrl.add_entry(t.unwrap()); }
	}
}
