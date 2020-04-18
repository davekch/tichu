mod combinations;
mod deck;
mod player;
mod tichugame;
mod tichuserver;

use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use tichuserver::TichuServer;

fn main() {
    // set up logger
    let _ = TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed);
    let mut server = TichuServer::new();
    server.main("127.0.0.1", "1001");
}
