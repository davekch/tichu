use crate::combinations::Trick;
use crate::deck::Card;

pub struct Player<'a> {
    hand: Vec<&'a Card>,
    stage: Trick<'a>, // here, cards are stored that the player is planning to play
    pub username: String,
}

impl<'a> Player<'a> {
    pub fn new(username: String) -> Player<'a> {
        Player {
            hand: Vec::new(),
            stage: Trick::new(),
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

    pub fn play(&mut self, trick: Option<&Trick>) -> Result<Trick, PlayerError> {
        // if the player is first, trick is None, else the own stage must top the trick
        match trick {
            None => {
                if self.stage.is_valid() {
                    Ok(self.stage.empty()) // give up ownership of the trick, new stage is now empty
                } else {
                    Err(PlayerError::NotValid)
                }
            }
            Some(trick) => {
                match self.stage.tops(trick) {
                    Some(true) => Ok(self.stage.empty()), // give up ownership of the trick, new stage is now empty
                    Some(false) => Err(PlayerError::TooLow),
                    None => Err(PlayerError::Incompatible),
                }
            }
        }
    }
}

pub enum PlayerError {
    NotValid,
    TooLow,
    Incompatible,
}
