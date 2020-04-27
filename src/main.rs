mod combinations;
mod deck;
mod player;
mod tichugame;
mod tichuserver;

use ctrlc;
use log::{error, info};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tichuserver::TichuServer;

fn main() {
    // set up logger
    let _ = TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed);

    let exit_condition = Arc::new(AtomicBool::new(false));
    let ex = exit_condition.clone();
    // setup behaviour on ctrl-c signal
    ctrlc::set_handler(move || {
        info!("received ctrl-c");
        ex.store(true, Ordering::SeqCst);
    }).expect("Could not set ctrl-c handler");
    let server = TichuServer::accept("127.0.0.1", "1001", exit_condition);
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
