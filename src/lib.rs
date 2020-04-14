mod combinations;
mod deck;
mod player;

use combinations::Trick;
use deck::Deck;
use player::Player;

pub struct TichuGame<'a> {
    players: [Player<'a>; 4],
    deck: Deck,
    pub current_player: u8,
    pub current_trick: Vec<Trick<'a>>,
    pub scores: Vec<(i16, i16)>,
}

impl<'a> TichuGame<'a> {
    pub fn new(usernames: [String; 4]) -> TichuGame<'a> {
        TichuGame {
            players: [
                Player::new(usernames[0].to_string()),
                Player::new(usernames[1].to_string()),
                Player::new(usernames[2].to_string()),
                Player::new(usernames[3].to_string()),
            ],
            deck: Deck::new(),
            current_player: 0,
            scores: vec![(0, 0)],
            current_trick: Vec::new(),
        }
    }

    pub fn deal(&'a mut self) {
        self.deck.shuffle();
        let hands = self.deck.deal();
        for i in 0..4 {
            self.players[i].take_new_hand(hands[0].to_vec());
        }
    }
}
