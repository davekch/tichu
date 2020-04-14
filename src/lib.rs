mod combinations;
mod deck;
mod player;

use combinations::Trick;
use deck::{Card, Deck};
use player::Player;

pub struct TichuGame {
    players: [Player; 4],
    pub current_player: u8,
    pub current_trick: Vec<Trick>,
    pub scores: Vec<(i16, i16)>,
}

impl TichuGame {
    pub fn new(usernames: [String; 4]) -> TichuGame {
        let mut deck = Deck::new();
        deck.shuffle();
        let hands = deck.deal();
        TichuGame {
            players: [
                Player::new(usernames[0].to_string(), hands[0].to_vec()),
                Player::new(usernames[1].to_string(), hands[1].to_vec()),
                Player::new(usernames[2].to_string(), hands[2].to_vec()),
                Player::new(usernames[3].to_string(), hands[3].to_vec()),
            ],
            current_player: 0,
            scores: vec![(0, 0)],
            current_trick: Vec::new(),
        }
    }
}
