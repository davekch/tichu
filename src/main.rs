mod combinations;
mod deck;
mod player;
mod tichugame;
mod tichuserver;

use log::error;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use tichuserver::TichuServer;

fn main() {
    // set up logger
    let _ = TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed);
    let server = TichuServer::accept("127.0.0.1", "1001");
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
