use crate::combinations::Trick;
use crate::deck::{Card, Deck};

pub struct TichuGame {
    deck: Deck,
    // holds the hands that are meant for players after dealing, None as soon as a player takes theirs
    hands: [Option<Vec<Card>>; 4],
    pub current_player: usize,
    player_points: [i16; 4],
    pub tricks: Vec<Trick>, // tricks in the middle of the table
    pub passes: u8, // number of times that players have passed (at 3, last_trick wins the round)
    pub scores: Vec<(i16, i16)>,
}

impl TichuGame {
    pub fn new() -> TichuGame {
        TichuGame {
            deck: Deck::new(),
            hands: [None, None, None, None],
            current_player: 0,
            player_points: [0, 0, 0, 0],
            passes: 0,
            scores: vec![(0, 0)],
            tricks: Vec::new(),
        }
    }

    pub fn shuffle_and_deal(&mut self) {
        self.deck.shuffle();
        let hands = self.deck.deal();
        for i in 0..4 {
            self.hands[i] = Some(hands[i].to_vec());
        }
    }

    pub fn take_hand(&mut self, i: usize) -> Option<Vec<Card>> {
        self.hands[i].take()
    }

    pub fn pass(&mut self) {
        // call this if a player doesn't want to play
        self.passes += 1;
        self.current_player = (self.current_player + 1) % 4;
        if self.passes == 3 {
            // if 3 players pass, the current player wins this round
            self.passes = 0;
            // collect all the points
            for trick in &self.tricks {
                self.player_points[self.current_player] += trick.points();
            }
            self.tricks = Vec::new();
        }
    }

    pub fn add_trick(&mut self, trick: Trick) {
        // players must make sure themselves that trick is valid
        self.tricks.push(trick);
        self.current_player = (self.current_player + 1) % 4;
        self.passes = 0;
    }

    pub fn get_current_trick(&self) -> Option<&Trick> {
        if self.tricks.len() > 0 {
            Some(&self.tricks[self.tricks.len() - 1])
        } else {
            None
        }
    }
}
