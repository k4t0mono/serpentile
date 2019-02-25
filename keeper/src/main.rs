#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

mod keeper;

use std::net::{SocketAddr, TcpListener, TcpStream, IpAddr};
use std::io::Read;
use serpentine::*;
use keeper::*;


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
        eprintln!("Usage: {} <mode> [log-level]", args[0]);
        panic!("Missing args");
    }

    let log_level = args.last().unwrap().parse::<usize>().unwrap_or(3);

    (log_level)
}


fn handle_client(mut stream: TcpStream, keeper: &mut Keeper) {
    let origin_ip = stream.peer_addr().unwrap().ip();
    trace!("New connection from {}", origin_ip);

    let mut buf: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buf).unwrap();

    match &buf[0] {
        0x01 => { process_listener(origin_ip, &buf[1..3], 1, keeper); },
        0x02 => { process_listener(origin_ip, &buf[1..3], 2, keeper); },
        0x03 => { process_listener(origin_ip, &buf[1..3], 3, keeper); },
        0x10 => { process_transaction(buf, keeper); },
        0x20 => { process_block(buf, keeper); },
        _ => ()
    };
}

fn process_listener(ip: IpAddr, port_: &[u8], op: u8, keeper: &mut Keeper) {
    debug!("New block found");

    let port: u16 = ((port_[0] as u16) << 8) + (port_[1] as u16);
    let sa = SocketAddr::new(ip, port);

    match op {
        1 => { keeper.add_block_listener(sa); },
        2 => { keeper.add_transaction_listener(sa); },
        3 => { keeper.add_block_transaction_listener(sa); },
        _ => (),
    };
}

fn process_block(buf: Vec<u8>, keeper: &mut Keeper) {
    debug!("New block recived");

    let b = match Block::deserialize(&buf[1..]) {
        Ok(v) => v,
        Err(e) => {
            debug!("Couldn't read a block. Error: {}", e);
            return;
        },
    };

    keeper.add_block(b);
}

fn process_transaction(buf: Vec<u8>, keeper: &mut Keeper) {
    debug!("New transaction recived");

    let t = match Transaction::deserialize(&buf[1..]) {
        Ok(v) => v,
        Err(e) => {
            debug!("Couldn't read a transaction. Error: {}", e);
            return;
        },
    };

    keeper.add_transaction(t);
}


fn main() {
    let (log_level) = parse_args();
    set_logger(log_level);

    let port: u16 =  match std::env::var("KEEPER_PORT") {
        Ok(n) => n.parse::<u16>().unwrap(),
        Err(_) => 0xe621,
    };

    info!("Starting keeper.rs at port: {}", port);

    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).unwrap();

    let mut keeper = Keeper::new();

    for stream in listener.incoming() {
        handle_client(stream.unwrap(), &mut keeper);
    }
}
