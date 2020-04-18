use crate::player::Player;
use crate::tichugame::TichuGame;
use bufstream::BufStream;
use log::{debug, error, info};
use std::io::BufRead;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct TichuServer {
    // Mutex<T> can be mutably accessed via a lock, Arc<T> allows multiple owners
    game: Arc<Mutex<TichuGame<'static>>>,
    join_handles: [Option<thread::JoinHandle<()>>; 4],
}

impl TichuServer {
    pub fn new() -> TichuServer {
        TichuServer {
            game: Arc::new(Mutex::new(TichuGame::new())),
            join_handles: [None, None, None, None],
        }
    }

    pub fn handle_connection(
        stream: TcpStream,
        player: Player<'static>,
        game_mutex: Arc<Mutex<TichuGame<'static>>>,
    ) {
        // clone the stream because BufStream::new() and lines() take ownership
        let stream = BufStream::new(stream);
        for line in stream.lines() {
            match line {
                Ok(msg) => info!("got message for {}: {}", player.username, msg),
                Err(e) => error!("Error while reading message for {}: {}", player.username, e),
            }
        }
    }

    pub fn main(&mut self, ip: &str, port: &str) {
        let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(l) => {
                info!("TichuServer listening on {}:{}", ip, port);
                l
            }
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        // accept first four incoming connections
        let mut i: usize = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if i < 4 {
                        self.add_connection(i, stream);
                        i += 1;
                    } else {
                        info!("connections complete, ready to start game");
                        break;
                    }
                }
                Err(e) => error!("{}", e),
            }
        }
        self.join_all();
        info!("quitting ...");
        drop(listener);
    }

    fn add_connection(&mut self, i: usize, stream: TcpStream) {
        let addr = stream.peer_addr().unwrap();
        // read username from stream
        let mut bufstream = BufStream::new(stream.try_clone().unwrap());
        let mut username = String::new();
        bufstream.read_line(&mut username).unwrap();
        // spawn a new thread where the new connection is checked for incoming messages
        info!("new connection with {} via {}", username.trim(), addr);
        let gameclone = Arc::clone(&self.game);
        let handle = thread::spawn(move || {
            TichuServer::handle_connection(
                stream,
                Player::new(username.trim().to_string()),
                gameclone,
            )
        });
        self.join_handles[i] = Some(handle);
    }

    fn join_all(&mut self) {
        for handle in &mut self.join_handles {
            match handle.take().unwrap().join() {
                Ok(_) => {},
                Err(_) => error!("Could not join thread"),
            }
        };
        info!("all treads joined");
    }
}
