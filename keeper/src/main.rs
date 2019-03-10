#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

mod config;
mod keeper;

use std::{env, process};
use std::net::{SocketAddr, TcpListener, TcpStream, IpAddr};
use std::io::Read;
use serpentine::*;
use keeper::*;
use config::Config;


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
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("Usage: wallet <port>");
        process::exit(1);
    });
    info!("Starting keeper.rs at port: {}", config.port);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).unwrap();

    let mut keeper = Keeper::new();

    for stream in listener.incoming() {
        handle_client(stream.unwrap(), &mut keeper);
    }
}
