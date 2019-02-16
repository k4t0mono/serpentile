#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use serpentine::utils::*;


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
	
	if args.len() < 2 {
		eprintln!("Usage: {} [log-level]", args[0]);
		panic!("Missing args");
	}

	let log_level = args.last().unwrap().parse::<usize>().unwrap_or(3);

	(log_level)
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
	debug!("New connection from {}", stream.peer_addr()?);

	let mut buf: [u8; 10] = [0; 10];
	stream.read(&mut buf)?;

	let t = Transaction::deserialize(buf)?;
	info!("Recived: {}", t);

	Ok(())
}

fn main() {
	let log_level = parse_args();
	set_logger(log_level);
	info!("Starting miner.rs");

	let listener = TcpListener::bind("0.0.0.0:34254").unwrap();
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => { handle_client(stream).unwrap(); },
			Err(e) => { panic!("{:?}", e); },
		}
	}
}
