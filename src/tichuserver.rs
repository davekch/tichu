use crate::tichugame::TichuGame;
use crate::player::Player;
use crate::deck::Deck;
use std::net::{TcpStream, TcpListener};
use bufstream::BufStream;
use log::{info, error};

pub struct Connection<'a> {
    player: Player<'a>,
    stream: TcpStream,
}

pub struct TichuServer<'a> {
    connections: [Option<Connection<'a>>; 4],
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
        for stream in listener.incoming() {
            let mut i: usize = 0;
            match stream {
                Ok(stream) => if i < 4 {
                    self.add_connection(i, stream);
                    i += 1
                } else {
                    info!("connections complete, ready to start game");
                    break;
                }
                Err(e) => {error!("{}", e); return;}
            }
        }
        // info!("quitting ...");
        // drop(listener);
    }

    fn add_connection(&mut self, i: usize, mut stream: TcpStream) {
        // read username from stream
        info!("new connection via {}", stream.peer_addr().unwrap());
    }
}
