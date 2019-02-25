use std::net::SocketAddr;


pub struct Config {
    pub n_messages: usize,
    pub keeper_addrs: Vec<SocketAddr>,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let n_messages = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("Didn't get a number of messages")
        };

        let keeper_addrs = match args.next() {
            Some(hf) => parse_hostfile(hf),
            None => vec!["127.0.0.1:58913".parse().unwrap()],
        };

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or(String::from("3"))
            .parse::<usize>()
            .unwrap();

        set_logger(log_level);

        Ok(Config{ n_messages, keeper_addrs })
    }

}


fn parse_hostfile(path: String) -> Vec<SocketAddr> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fl = File::open(path)
        .expect("Could not read the lines");

    let a: Vec<String> = BufReader::new(fl)
        .lines()
        .map(|l| l.expect("Could not read the lines"))
        .collect();

    let mut addrs: Vec<SocketAddr> = vec![];
    for addr in a.iter() {
        addrs.push(addr.parse().expect(""));
    }

    addrs
}


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

