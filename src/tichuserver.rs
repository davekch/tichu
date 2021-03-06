use crate::deck::{Card, SpecialKind};
use crate::player::{Player, PlayerError};
use crate::tichugame::{TichuGame, RoundStatus};
use bufstream::BufStream;
use log::{debug, error, info, warn};
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

struct TichuConnection {
    game: Mutex<TichuGame>,
    streams: [Mutex<TcpStream>; 4],
}

impl TichuConnection {
    pub fn new(connections: [Mutex<TcpStream>; 4]) -> TichuConnection {
        TichuConnection {
            game: Mutex::new(TichuGame::new()),
            streams: connections,
        }
    }

    pub fn handle_connection(
        &self,
        player_index: usize,
    ) {
        // say hello (this message also tells the client that every other player is connected)
        self.answer_ok(player_index);
        // create a read-only bufstream of this connection's TcpStream
        let mut readstream: BufStream<TcpStream>;
        {
            let stream = self.streams[player_index].lock().unwrap();
            // clone the stream because BufStream::new() and lines() take ownership
            // use readstream to iterate over incoming lines, this has the advantage
            // that it blocks the thread until there is a new line as opposed to
            // loop { stream.read() }.
            readstream = BufStream::new(stream.try_clone().unwrap());
        }
        // get the username first and create a player instance
        let mut username = String::new();
        readstream.read_line(&mut username).unwrap();
        let mut player = Player::new(username.trim().to_string());

        // main loop waiting for commands
        for line in readstream.lines() {
            if let Ok(msg) = line {
                debug!("got message from {}: {}", player.username, msg);
                // check for all the possible messages
                if msg == "takecards" {
                    // acquire the lock to self.game
                    let mut game = self.game.lock().unwrap();
                    match game.take_hand(player_index) {
                        Some(h) => {
                            self.answer_msg(player_index, &format_hand(&h));
                            // check if this player has the One
                            if h.contains(&Card::special(SpecialKind::One)) {
                                game.current_player = player_index;
                                self.send_push(player_index, "yourturn", "");
                            }
                            player.take_new_hand(h);
                        }
                        _ => {
                            debug!("a client tried to take a hand that does not exist");
                            self.answer_err(
                                player_index,
                                "there is no hand for you at the moment",
                            );
                        }
                    };
                // lock gets released at end of this scope
                } else if msg.starts_with("play") && self.require_turn(player_index) {
                    let args = parse_command_parameters(&msg);
                    // check if it's the player's turn
                    let mut game = self.game.lock().unwrap();
                    let current_trick = game.get_current_trick();
                    // let the player play against the current trick
                    let played = player.play(current_trick, &args);
                    match played {
                        Ok(trick) => {
                            self.answer_ok(player_index);
                            self.send_push_to_all("newtrick", &format_hand(&trick.cards));
                            debug!("the current trick is {:?}", &trick);
                            game.add_trick(trick);
                            self.continue_round(game, &player, player_index);
                        }
                        Err(PlayerError::NotValid) => self.answer_err(
                            player_index,
                            "Your cards don't form a valid trick",
                        ),
                        Err(PlayerError::TooLow) => self.answer_err(
                            player_index,
                            "Your trick is lower than the current trick",
                        ),
                        Err(PlayerError::Incompatible) => self.answer_err(
                            player_index,
                            "Your trick is incompatible with the current trick",
                        ),
                        Err(PlayerError::InvalidCard) => {
                            error!("Player {} tried to play a card that is not in their hand anymore", player_index);
                            self.answer_err(player_index, "something went wrong on the server side");
                        }
                    }
                } else if msg == "pass" && self.require_turn(player_index) {
                    let mut game = self.game.lock().unwrap();
                    game.pass();
                    self.answer_ok(player_index);
                    self.continue_round(game, &player, player_index);
                } else {
                    warn!("received invalid message from {}: {}", player.username, msg);
                    // self.answer_err(player_index, "invalid command");
                }
            } else if let Err(e) = line {
                error!("Error while reading message for {}: {}", player.username, e);
            }
        }
    }

    fn continue_round(&self, mut game: MutexGuard<TichuGame>, player: &Player, player_index: usize) {
        // this function rotates the current player and  checks for all
        // possible cases after a player finished their move
        let status = game.next();
        if status == RoundStatus::TrickWin {
            self.send_push_to_all("cleartable", "");
        }
        if !player.has_cards() {
            match game.mark_finished(player_index) {
                RoundStatus::FinishRound => {
                    game.shuffle_and_deal();
                    self.send_push_to_all(&"cleartable", "");
                    self.send_push_to_all(&"clearcards", "");
                    debug!("this round is finished! new points: {:?}", game.get_current_score());
                }
                RoundStatus::Continue => {}
                _ => {}
            };
        }
        self.send_push(game.current_player, "yourturn", "");
    }

    fn answer_ok(&self, index: usize) {
        self.send(index, "ok:");
    }

    fn answer_msg(&self, index: usize, msg: &str) {
        self.send(index, &format!("ok:{}", msg));
    }

    fn answer_err(&self, index: usize, msg: &str) {
        self.send(index, &format!("err:{}", msg));
    }

    fn send(&self, index: usize, msg: &str) {
        // acquire lock for this stream
        let mut stream = self.streams[index].lock().unwrap();
        match stream.write(format!("{}\n", msg).as_bytes()) {
            Ok(_) => {}
            Err(e) => error!("could not send message '{}' to player {}: {}", msg, index, e),
        };
    }

    pub fn send_to_stream(stream: &mut TcpStream, msg: &str) {
        match stream.write(format!("{}\n", msg).as_bytes()) {
            Ok(_) => {}
            Err(e) => error!("could not send message '{}': {}", msg, e),
        };
    }

    fn send_push_to_all(&self, topic: &str, msg: &str) {
        // send a push message to all clients in self.streams
        for i in 0..4 {
            self.send_push(i, topic, msg);
        }
    }

    fn send_push(&self, index: usize, topic: &str, msg: &str) {
        self.send(index, &format!("push:{}:{}", topic, msg));
    }

    fn require_turn(&self, player_index: usize) -> bool {
        // check if it's the player's turn
        // clients should themselves forbid to send commands if it's not their turn
        // because checking on server side requires the lock on self.game
        let game = self.game.lock().unwrap();
        if game.current_player == player_index {
            true
        } else {
            self.answer_err(player_index, "It's not your turn");
            false
        }
    }
}

pub struct TichuServer {
    // Mutex<T> can be mutably accessed via a lock, Arc<T> allows multiple owners
    inner: Arc<TichuConnection>,
    listener: TcpListener,
    join_handles: [Option<thread::JoinHandle<()>>; 4],
}

impl TichuServer {
    pub fn accept(ip: &str, port: &str) -> Result<TichuServer, std::io::Error> {
        let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(l) => {
                info!("TichuConnection listening on {}:{}", ip, port);
                l
            }
            Err(e) => return Err(e),
        };
        // accept first four incoming connections
        let mut streams: [Option<TcpStream>; 4] = [None, None, None, None];
        let mut i: usize = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap();
                    info!("new connection with {}", addr);
                    streams[i] = Some(stream);
                    i += 1;
                    if i == 4 {
                        info!("connections complete, ready to start game");
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }
        let connections = [
            Mutex::new(streams[0].take().unwrap()),
            Mutex::new(streams[1].take().unwrap()),
            Mutex::new(streams[2].take().unwrap()),
            Mutex::new(streams[3].take().unwrap()),
        ];
        Ok(TichuServer {
            inner: Arc::new(TichuConnection::new(connections)),
            listener: listener,
            join_handles: [None, None, None, None],
        })
    }

    pub fn main(&mut self) {
        // shuffle and deal cards for everyone
        {
            let innerclone = self.inner.clone();
            let mut game = innerclone.game.lock().unwrap();
            game.shuffle_and_deal();
        }
        // spawn a thread for each player and listen to their incoming messages
        for i in 0..4 {
            let innerclone = self.inner.clone();
            let handle = thread::spawn(move || {
                innerclone.handle_connection(i)
            });
            self.join_handles[i] = Some(handle);
        }
        // let the threads do their job and wait til everything ends
        self.join_all();
    }

    pub fn stop(self) {
        info!("quitting ...");
        drop(self.listener);
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

fn parse_command_parameters(command: &str) -> Vec<usize> {
    // parse something like "command 1 2" into (1, 2)
    let mut parts = command.split_whitespace();
    // ignore first part
    parts.next();
    let mut args: Vec<usize> = Vec::new();
    for p in parts {
        args.push(p.parse().unwrap());
    }
    return args
}
