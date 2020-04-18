use crate::deck::Card;
use crate::player::Player;
use crate::tichugame::TichuGame;
use bufstream::BufStream;
use log::{debug, error, info, warn};
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct TichuServer {
    // Mutex<T> can be mutably accessed via a lock, Arc<T> allows multiple owners
    game: Arc<Mutex<TichuGame>>,
    join_handles: [Option<thread::JoinHandle<()>>; 4],
}

impl TichuServer {
    pub fn new() -> TichuServer {
        TichuServer {
            game: Arc::new(Mutex::new(TichuGame::new())),
            join_handles: [None, None, None, None],
        }
    }

    fn handle_connection(
        player_index: usize,
        mut writestream: TcpStream,
        mut player: Player,
        game_mutex: Arc<Mutex<TichuGame>>,
    ) {
        // clone the stream because BufStream::new() and lines() take ownership
        // use readstream to iterate over incoming lines, this has the advantage
        // that it blocks the thread until there is a new line as opposed to
        // loop { stream.read() }. writestream is used to send messages back
        let mut readstream = BufStream::new(writestream.try_clone().unwrap());
        for line in readstream.lines() {
            if let Ok(msg) = line {
                debug!("got message for {}: {}", player.username, msg);
                // check for all the possible messages
                if msg == "takecards" {
                    // acquire the lock to game_mutex
                    let mut game = game_mutex.lock().unwrap();
                    match game.take_hand(player_index) {
                        // TODO send message back to client
                        Some(h) => {
                            TichuServer::answer_msg(&mut writestream, &format_hand(&h));
                            player.take_new_hand(h);
                        }
                        _ => {
                            error!("a client tried to take a hand that does not exist");
                            TichuServer::answer_err(
                                &mut writestream,
                                "there is no hand for you at the moment",
                            );
                        }
                    };
                // lock gets released at end of this scope
                } else if msg == "deal" {
                    let mut game = game_mutex.lock().unwrap();
                    // TODO: only allow this if there's a new round
                    if game.current_player == player_index {
                        game.shuffle_and_deal();
                        TichuServer::answer_ok(&mut writestream);
                    } else {
                        TichuServer::answer_err(&mut writestream, "it's not your turn");
                    }
                } else if msg.starts_with("stage") {
                    let (i, j) = parse_command_parameters(&msg);
                    player.stage(i, j);
                    TichuServer::answer_ok(&mut writestream);
                } else if msg.starts_with("unstage") {
                    let (i, j) = parse_command_parameters(&msg);
                    player.unstage(i, j);
                    TichuServer::answer_ok(&mut writestream);
                } else {
                    warn!("received invalid message: {}", msg);
                    TichuServer::answer_err(&mut writestream, "invalid command");
                }
            } else if let Err(e) = line {
                error!("Error while reading message for {}: {}", player.username, e);
            }
        }
    }

    fn answer_ok(stream: &mut TcpStream) {
        TichuServer::answer(stream, "ok:");
    }

    fn answer_msg(stream: &mut TcpStream, msg: &str) {
        TichuServer::answer(stream, &format!("ok:{}", msg));
    }

    fn answer_err(stream: &mut TcpStream, msg: &str) {
        TichuServer::answer(stream, &format!("err:{}", msg));
    }

    fn answer(stream: &mut TcpStream, msg: &str) {
        match stream.write(format!("{}\n", msg).as_bytes()) {
            Ok(_) => {}
            Err(e) => error!("could not send message '{}': {}", msg, e),
        };
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
        TichuServer::answer_ok(&mut stream);
        info!("new connection with {} via {}", username.trim(), addr);
        // spawn a new thread where the new connection is checked for incoming messages
        let gameclone = Arc::clone(&self.game);
        let handle = thread::spawn(move || {
            TichuServer::handle_connection(
                i,
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
