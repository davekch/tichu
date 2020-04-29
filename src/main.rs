mod combinations;
mod deck;
mod player;
mod tichugame;
mod tichuserver;

#[macro_use]
extern crate clap;
use clap::App;
use log::error;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use tichuserver::TichuServer;

fn main() {
    // set up logger
    let _ = TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed);
    // set up argument parser
    let options = load_yaml!("cli.yml");
    let args = App::from_yaml(options).get_matches();
    let ip = args.value_of("ip_address").unwrap_or("127.0.0.1");
    let port = args.value_of("port").unwrap_or("1001");

    let server = TichuServer::accept(ip, port);
    match server {
        Ok(mut server) => {
            server.main();
            server.stop();
        }
        Err(e) => {
            error!("{}", e);
            return;
        }
    }
}
