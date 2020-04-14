use crate::combinations::Trick;
use crate::deck::Card;

pub struct Player {
    hand: Vec<Card>,
    stage: Trick, // here, cards are stored that the player is planning to play
    points: i16, // the points in the tricks that the player has during one round
    pub username: String,
}

impl Player {
    pub fn new(username: String, hand: Vec<Card>) -> Player {
        Player {
            hand: hand,
            stage: Trick::new(),
            points: 0,
            username: username,
        }
    }

    pub fn stage(&mut self, i: usize, j: usize) {
        // move card ith card in hand to j in stage
        self.stage.insert(j, self.hand.remove(i));
    }

    pub fn unstage(&mut self, i: usize, j: usize) {
        // move card ith card in stage to j in hand
        self.hand.insert(j, self.stage.remove(i));
    }

    // to do change borrow all cards from deck

    // pub fn play(mut self) -> Option<Trick> {
    //     // give up ownership of the trick, new stage is now empty
    //     let trick = self.stage;
    //     self.stage = Trick::new();
    //     Some(trick)
    // }
}
