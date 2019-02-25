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

fn parse_args() -> (usize, usize) {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <n-messages> [log-level]", args[0]);
        panic!("Missing args");
    }

    let n = args[1].parse::<usize>().unwrap_or(5);

    let c = args.last().unwrap().chars().last().unwrap().to_string();
    let log_level = if args.len() > 3 { c.parse::<usize>().unwrap_or(3) } else { 3 };

    (n, log_level)
}


fn main() {
    let (n, log_level) = parse_args();
    set_logger(log_level);
    info!("Starting wallet.rs");

    let id: u16 = rand::thread_rng().gen();
    let wallet = Wallet::new(id);

    for i in 0..n {
        wallet.new_transaction(0x0032 + ((i as u16) << 8), 20.0 + (i as f32) / 10.0);
        thread::sleep(time::Duration::from_millis(750));
    }
}
