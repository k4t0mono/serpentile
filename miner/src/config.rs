use std::net::SocketAddr;


pub struct Config {
    pub port: u16,
    pub keeper_addr: SocketAddr,
    pub mode: usize,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let port = match args.next() {
            Some(s) => s.parse::<u16>().unwrap(),
            None => 0xE721,
        };

        let keeper_addr = match args.next() {
            Some(s) => s.parse().unwrap(),
            None => "127.0.0.1:58913".parse().unwrap(),
        };

        let mode = match args.next() {
            Some(m) => m.parse::<usize>().unwrap(),
            None => 0,
        };

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or(String::from("3"))
            .parse::<usize>()
            .unwrap();

        set_logger(log_level);

        Ok(Config{ keeper_addr, mode, port })
    }

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

