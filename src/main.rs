mod combinations;
mod deck;
mod player;
mod tichugame;
mod tichuserver;

use tichuserver::TichuServer;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode};

fn main() {
    // set up logger
    let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
    let mut server = TichuServer::new();
    server.main("127.0.0.1", "1001");
}
