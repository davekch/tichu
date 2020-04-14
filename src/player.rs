use crate::combinations::Trick;
use crate::deck::Card;

pub struct Player<'a> {
    hand: Vec<&'a Card>,
    stage: Trick<'a>, // here, cards are stored that the player is planning to play
    points: i16,      // the points in the tricks that the player has during one round
    pub username: String,
}

impl<'a> Player<'a> {
    pub fn new(username: String) -> Player<'a> {
        Player {
            hand: Vec::new(),
            stage: Trick::new(),
            points: 0,
            username: username,
        }
    }

    pub fn take_new_hand(&mut self, hand: Vec<&'a Card>) {
        self.hand = hand;
    }

    pub fn stage(&mut self, i: usize, j: usize) {
        // move card ith card in hand to j in stage
        self.stage.insert(j, self.hand.remove(i));
    }

    pub fn unstage(&mut self, i: usize, j: usize) {
        // move card ith card in stage to j in hand
        self.hand.insert(j, self.stage.remove(i));
    }

    pub fn play(&mut self, trick: &Trick) -> Option<Trick> {
        match self.stage.tops(trick) {
            Some(true) => Some(self.stage.empty()), // give up ownership of the trick, new stage is now empty
            _ => None,
        }
    }
}
