#[macro_use] extern crate log;
extern crate simplelog;
extern crate serpentine;

mod config;
mod ctrl;

use std::{env, process};
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr,};
use std::io::{Read, Write};
use std::option::Option;
use serpentine::Transaction;
use ctrl::*;
use config::Config;


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

fn normal_mode(port: u16) {
    info!("Starting miner.rs at port {}", port);
    let mut ctrl = Ctrl::new(5);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let listener = TcpListener::bind(socket).unwrap();
    for stream in listener.incoming() {
        let t = handle_client(stream.unwrap());
        if t.is_some() { ctrl.add_entry(t.unwrap()); }
    }
}

fn register_listener(addr: SocketAddr, port: u16) {
    let mut stream = TcpStream::connect(addr).unwrap();

    let ph = (port >> 8) as u8;
    let pl = port as u8;

    let buff = vec![0x03, ph, pl];

    stream.write(&buff[..]).unwrap();
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("Usage: miner <port> [keeper-addr] [mode]");
        process::exit(1);
    });

    register_listener(config.keeper_addr, config.port);

    match config.mode {
        1 => { debug_mode(); }
        _ => { normal_mode(config.port); }
    }
}
