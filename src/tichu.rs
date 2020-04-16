mod combinations;
mod deck;
mod player;

use combinations::Trick;
use deck::Deck;
use player::Player;

pub struct TichuGame<'a> {
    pub current_player: usize,
    player_points: [i16; 4],
    pub tricks: Vec<Trick<'a>>,  // tricks in the middle of the table
    pub passes: u8, // number of times that players have passed (at 3, last_trick wins the round)
    pub scores: Vec<(i16, i16)>,
}

impl<'a> TichuGame<'a> {
    pub fn new() -> TichuGame<'a> {
        TichuGame {
            current_player: 0,
            player_points: [0, 0, 0, 0],
            passes: 0,
            scores: vec![(0, 0)],
            tricks: Vec::new(),
        }
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

    pub fn play(&mut self, trick: Trick<'a>) {
        // players must make sure themselves that trick is valid
        self.tricks.push(trick);
        self.current_player = (self.current_player + 1) % 4;
        self.passes = 0;
    }
}
