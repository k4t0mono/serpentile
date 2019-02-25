#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

mod ctrl;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::option::Option;
use serpentine::Transaction;
use ctrl::*;
use rand::Rng;


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


fn parse_args() -> (String, usize, usize) {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <keeper-addr> <mode> [log-level]", args[0]);
        panic!("Missing args");
    }

    let keeper_addr = args[1].to_string();

    let mode = args[2].parse::<usize>().unwrap(); 

    let log_level = args.last().unwrap().parse::<usize>().unwrap_or(3);

    (keeper_addr, mode, log_level)
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

fn debug_mode() {
    let mut ctrl = Ctrl::new(5);

    for i in 0..5 {
        let from = 0xe621;
        let to = 0x0032 + ((i as u16) << 8);
        let value = 20.0 + (i as f32) / 10.0;

        ctrl.add_entry(Transaction::new(from, to, value));
    }
}

fn normal_mode() {
    let mut ctrl = Ctrl::new(5);

    let listener = TcpListener::bind("0.0.0.0:34254").unwrap();
    for stream in listener.incoming() {
        let t = handle_client(stream.unwrap());
        if t.is_some() { ctrl.add_entry(t.unwrap()); }
    }
}

fn register_listener(addr: String) {
    let mut stream = TcpStream::connect(addr).unwrap();

    let mut rng = rand::thread_rng();
    let port: u8 = rng.gen_range(0x00, 0xff);

    let buff = vec![0x03, 0xe7, port];

    stream.write(&buff[..]).unwrap();
}

fn main() {
    let (keeper_addr, mode, log_level) = parse_args();
    set_logger(log_level);
    info!("Starting miner.rs");

    register_listener(keeper_addr);

    match mode {
        1 => { debug_mode(); }
        _ => { normal_mode(); }
    }
}
