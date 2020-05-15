use crate::combinations::Trick;
use crate::deck::Card;
use std::collections::HashMap;

pub struct Player {
    hand: HashMap<usize, Card>,
    pub username: String,
}

impl Player {
    pub fn new(username: String) -> Player {
        Player {
            hand: HashMap::new(),
            username: username,
        }
    }

    pub fn take_new_hand(&mut self, hand: Vec<Card>) {
        // store each card in a hash map with an index works as an identifyer
        let mut id = 0;
        for c in hand {
            self.hand.insert(id, c);
            id += 1;
        }
    }

    pub fn play(&mut self, trick_to_top: Option<&Trick>, cards: &Vec<usize>) -> Result<Trick, PlayerError> {
        // build the own trick
        let mut own_trick = Trick::new();
        for i in cards {
            let card = self.hand.get(i);
            match card {
                Some(c) => own_trick.push(*c),
                None => return Err(PlayerError::InvalidCard),
            }
        }
        // if the player is first, trick is None, else the own stage must top the trick
        match trick_to_top {
            None => {
                if own_trick.is_valid() {
                    self.remove_cards(cards);
                    Ok(own_trick)
                } else {
                    Err(PlayerError::NotValid)
                }
            }
            Some(trick) => {
                match own_trick.tops(trick) {
                    Some(true) => {
                        self.remove_cards(cards);
                        Ok(own_trick)
                    }
                    Some(false) => Err(PlayerError::TooLow),
                    None => Err(PlayerError::Incompatible),
                }
            }
        }
    }

    fn remove_cards(&mut self, cards: &Vec<usize>) {
        for i in cards {
            self.hand.remove(i);
        }
    }

    pub fn has_cards(&self) -> bool {
        self.hand.len() > 0
    }
}

pub enum PlayerError {
    InvalidCard,
    NotValid,
    TooLow,
    Incompatible,
}
