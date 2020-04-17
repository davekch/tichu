use crate::tichugame::TichuGame;
use crate::player::Player;
use crate::deck::Deck;
use std::net::{TcpStream, TcpListener};
use bufstream::BufStream;
use std::io::BufRead;
use std::thread;
use log::{debug, info, error};

pub struct Connection<'a> {
    player: Player<'a>,
    stream: TcpStream,
}

impl<'a> Connection<'a> {
    pub fn handle_connection(&mut self) {
        // clone the stream because BufStream::new() and lines() take ownership
        let stream = BufStream::new(self.stream.try_clone().unwrap());
        for line in stream.lines() {
            match line {
                Ok(msg) => info!("got message for {}: {}", self.player.username, msg),
                Err(e) => error!("Error while reading message for {}: {}", self.player.username, e),
            }
        }
    }
}

pub struct TichuServer<'a> {
    connections: [Option<String>; 4], // array of addresses to players
    game: TichuGame<'a>,
    deck: Deck,
}

impl<'a> TichuServer<'a> {
    pub fn new() -> TichuServer<'a> {
        TichuServer {
            connections: [None, None, None, None],
            game: TichuGame::new(),
            deck: Deck::new(),
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
                Ok(stream) => if i < 4 {
                    self.add_connection(i, stream);
                    i += 1;
                } else {
                    info!("connections complete, ready to start game");
                    break;
                }
                Err(e) => error!("{}", e),
            }
        }
        // info!("quitting ...");
        // drop(listener);
    }

    fn add_connection(&mut self, i: usize, stream: TcpStream) {
        let addr = stream.peer_addr().unwrap();
        // read username from stream
        let mut bufstream = BufStream::new(stream.try_clone().unwrap());
        let mut username = String::new();
        bufstream.read_line(&mut username).unwrap();
        let mut new_connection = Connection{
            player: Player::new(username.trim().to_string()),
            stream: stream,
        };
        self.connections[i] = Some(addr.to_string());
        // spawn a new thread where the new connection is checked for incoming messages
        thread::spawn(move || new_connection.handle_connection());
        info!("new connection with {} via {}", username.trim(), addr);
    }
}
