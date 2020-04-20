use crate::deck::Card;
use crate::player::{Player, PlayerError};
use crate::tichugame::TichuGame;
use bufstream::BufStream;
use log::{debug, error, info, warn};
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct TichuConnection {
    game: Mutex<TichuGame>,
}

impl TichuConnection {
    pub fn new() -> TichuConnection {
        TichuConnection {
            game: Mutex::new(TichuGame::new()),
        }
    }

    pub fn handle_connection(
        &self,
        player_index: usize,
        mut writestream: TcpStream,
        mut player: Player,
    ) {
        // clone the stream because BufStream::new() and lines() take ownership
        // use readstream to iterate over incoming lines, this has the advantage
        // that it blocks the thread until there is a new line as opposed to
        // loop { stream.read() }. writestream is used to send messages back
        let readstream = BufStream::new(writestream.try_clone().unwrap());
        for line in readstream.lines() {
            if let Ok(msg) = line {
                debug!("got message for {}: {}", player.username, msg);
                // check for all the possible messages
                if msg == "takecards" {
                    // acquire the lock to self.game
                    let mut game = self.game.lock().unwrap();
                    match game.take_hand(player_index) {
                        // TODO send message back to client
                        Some(h) => {
                            TichuConnection::answer_msg(&mut writestream, &format_hand(&h));
                            player.take_new_hand(h);
                        }
                        _ => {
                            error!("a client tried to take a hand that does not exist");
                            TichuConnection::answer_err(
                                &mut writestream,
                                "there is no hand for you at the moment",
                            );
                        }
                    };
                    // lock gets released at end of this scope
                } else if msg == "deal" && self.require_turn(player_index, &mut writestream) {
                    let mut game = self.game.lock().unwrap();
                    // TODO: only allow this if there's a new round
                    if game.current_player == player_index {
                        game.shuffle_and_deal();
                        TichuConnection::answer_ok(&mut writestream);
                    } else {
                        TichuConnection::answer_err(&mut writestream, "it's not your turn");
                    }
                } else if msg.starts_with("stage") {
                    let (i, j) = parse_command_parameters(&msg);
                    player.stage(i, j);
                    TichuConnection::answer_ok(&mut writestream);
                } else if msg.starts_with("unstage") {
                    let (i, j) = parse_command_parameters(&msg);
                    player.unstage(i, j);
                    TichuConnection::answer_ok(&mut writestream);
                } else if msg == "play" && self.require_turn(player_index, &mut writestream) {
                    // check if it's the player's turn
                    let mut game = self.game.lock().unwrap();
                    let current_trick = game.get_current_trick();
                    // let the player play against the current trick
                    let played = player.play(current_trick);
                    match played {
                        Ok(trick) => {
                            game.add_trick(trick);
                            TichuConnection::answer_ok(&mut writestream);
                            debug!("the current trick is {:?}", game.get_current_trick());
                            // TODO: notify the others
                        },
                        Err(PlayerError::NotValid) => TichuConnection::answer_err(
                            &mut writestream,
                            "Your cards don't form a valid trick",
                        ),
                        Err(PlayerError::TooLow) => TichuConnection::answer_err(
                            &mut writestream,
                            "Your trick is lower than the current trick",
                        ),
                        Err(PlayerError::Incompatible) => TichuConnection::answer_err(
                            &mut writestream,
                            "Your trick is incompatible with the current trick",
                        ),
                    }
                } else {
                    warn!("received invalid message from {}: {}", player.username, msg);
                    TichuConnection::answer_err(&mut writestream, "invalid command");
                }
            } else if let Err(e) = line {
                error!("Error while reading message for {}: {}", player.username, e);
            }
        }
    }

    fn answer_ok(stream: &mut TcpStream) {
        TichuConnection::answer(stream, "ok:");
    }

    fn answer_msg(stream: &mut TcpStream, msg: &str) {
        TichuConnection::answer(stream, &format!("ok:{}", msg));
    }

    fn answer_err(stream: &mut TcpStream, msg: &str) {
        TichuConnection::answer(stream, &format!("err:{}", msg));
    }

    fn answer(stream: &mut TcpStream, msg: &str) {
        match stream.write(format!("{}\n", msg).as_bytes()) {
            Ok(_) => {}
            Err(e) => error!("could not send message '{}': {}", msg, e),
        };
    }

    fn require_turn(&self, player_index: usize, stream: &mut TcpStream) -> bool {
        // check if it's the player's turn
        // clients should themselves forbid to send commands if it's not their turn
        // because checking on server side requires the lock on self.game
        let game = self.game.lock().unwrap();
        if game.current_player == player_index {
            true
        } else {
            TichuConnection::answer_err(stream, "It's not your turn");
            false
        }
    }
}

pub struct TichuServer {
    // Mutex<T> can be mutably accessed via a lock, Arc<T> allows multiple owners
    inner: Arc<TichuConnection>,
    join_handles: [Option<thread::JoinHandle<()>>; 4],
}

impl TichuServer {
    pub fn new() -> TichuServer {
        TichuServer {
            inner: Arc::new(TichuConnection::new()),
            join_handles: [None, None, None, None],
        }
    }

    pub fn main(&mut self, ip: &str, port: &str) {
        let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(l) => {
                info!("TichuConnection listening on {}:{}", ip, port);
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
                    self.add_connection(i, stream);
                    i += 1;
                    if i == 4 {
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

    fn add_connection(&mut self, i: usize, mut stream: TcpStream) {
        let addr = stream.peer_addr().unwrap();
        // read username from stream
        let mut bufstream = BufStream::new(stream.try_clone().unwrap());
        let mut username = String::new();
        bufstream.read_line(&mut username).unwrap();
        // if everything went well, say hello to the client
        TichuConnection::answer_ok(&mut stream);
        info!("new connection with {} via {}", username.trim(), addr);
        // spawn a new thread where the new connection is checked for incoming messages
        let innerclone = self.inner.clone();
        let handle = thread::spawn(move || {
            innerclone.handle_connection(i, stream, Player::new(username.trim().to_string()))
        });
        self.join_handles[i] = Some(handle);
    }

    fn join_all(&mut self) {
        for handle in &mut self.join_handles {
            match handle.take().unwrap().join() {
                Ok(_) => {}
                Err(_) => error!("Could not join thread"),
            }
        }
        info!("all treads joined");
    }
}

fn format_hand(hand: &Vec<Card>) -> String {
    let mut str = String::new();
    for card in hand {
        str += &format!("{},", card.to_string());
    }
    str.to_string()
}

fn parse_command_parameters(command: &str) -> (usize, usize) {
    // parse something like "command 1 2" into (1, 2)
    let mut parts = command.split_whitespace();
    // ignore first part
    parts.next();
    let num1: usize = parts.next().unwrap().parse().unwrap();
    let num2: usize = parts.next().unwrap().parse().unwrap();
    (num1, num2)
}
