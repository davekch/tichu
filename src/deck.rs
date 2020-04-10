use strum::IntoEnumIterator; // iterate over static enum
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, EnumIter)]
pub enum RegularKind {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, EnumIter)]
pub enum SpecialKind {
    Dragon,
    Phoenix,
    Dog,
    One,
}

#[derive(Debug, Clone, Hash)]
pub enum Kind {
    Special(SpecialKind),
    Regular(RegularKind),
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            // the phoenix is "equal" to all regular cards
            Kind::Special(SpecialKind::Phoenix) => {
                match other {
                    Kind::Regular(_) => true,
                    Kind::Special(o) => *o == SpecialKind::Phoenix
                }
            },
            Kind::Regular(s) => {
                match other {
                    Kind::Special(SpecialKind::Phoenix) => true,
                    Kind::Regular(o) => s == o,
                    _ => false
                }
            },
            Kind::Special(s) => {
                match other {
                    Kind::Regular(_) => false,
                    Kind::Special(o) => s == o
                }
            }
        }
    }
}

impl Eq for Kind {}


#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    None,
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Card {
    pub kind: Kind,
    pub color: Color,
    pub rank: i8,
    pub value: i8,
}

impl Card {
    pub fn new(kind: Kind, color: Color) -> Card {
        let mut c = color;  // change to None if kind is special
        let mut rank = 0;
        let mut value = 0;
        match &kind {
            Kind::Special(k) => {
                c = Color::None;
                match k {
                    SpecialKind::Phoenix => {rank = 0; value = -25;},
                    SpecialKind::Dragon => {rank = 14; value = 25},
                    _ => {}  // one and dog have rank and value 0
                }
            },
            Kind::Regular(k) => {
                rank = (*k as i8) + 2;
                match k {
                    RegularKind::Five => value = 5,
                    RegularKind::Ten => value = 10,
                    RegularKind::King => value = 10,
                    _ => {}
                }
            }
        };

        // return a card
        Card {
            kind: kind,
            color: c,
            rank: rank,
            value: value
        }
    }
}


pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Vec::new();
        // add all regular cards to deck
        for color in &[Color::Green, Color::Red, Color::Blue, Color::Black] {
            for kind in RegularKind::iter() {
                deck.push(Card::new(Kind::Regular(kind), *color));
            }
        }
        // add special cards to deck
        for kind in SpecialKind::iter() {
            deck.push(Card::new(Kind::Special(kind), Color::None));
        }
        Deck { cards: deck }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> [Vec<Card>; 4] {
        let mut hands = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for _i in 0..13 {
            for hand in &mut hands {
                // deal one card at a time
                let card = self.cards.pop();
                match card {
                    Some(c) => hand.push(c),
                    _ => {}  // never happens on a full deck anyway
                };
            }
        }
        hands
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_new_deck_length() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_new_deck_unique() {
        let deck = Deck::new();
        // remove dublicates
        let unique_deck: Vec<Card> = deck.cards
            .clone()
            .into_iter()
            .unique()
            .collect();
        assert_eq!(deck.cards, unique_deck);
    }

    #[test]
    fn test_deck_totalvalue() {
        // test if the total value of all cards is 100
        let deck = Deck::new();
        let mut total = 0;
        for card in &deck.cards {
            total += card.value;
        }
        assert_eq!(total, 100);
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new();
        let hands = deck.deal();
        // check if all cards are used
        assert_eq!(deck.cards.len(), 0);
        for hand in &hands {
            // check if each player has 13 cards
            assert_eq!(hand.len(), 13);
        }
    }

    #[test]
    fn test_equal_kinds() {
        // check two equals
        assert_eq!(
            Kind::Regular(RegularKind::Three),
            Kind::Regular(RegularKind::Three)
        );
        // check phoenix
        assert_eq!(
            Kind::Special(SpecialKind::Phoenix),
            Kind::Regular(RegularKind::Three)
        );
        assert_eq!(
            Kind::Regular(RegularKind::Three),
            Kind::Special(SpecialKind::Phoenix)
        );
        // check inequality between regular kinds
        assert_ne!(
            Kind::Regular(RegularKind::Three),
            Kind::Regular(RegularKind::Queen)
        );
        // check inequality between regular and special kind
        assert_ne!(
            Kind::Special(SpecialKind::One),
            Kind::Regular(RegularKind::Queen)
        );
        // check inequality between two special kinds
        assert_ne!(
            Kind::Special(SpecialKind::Phoenix),
            Kind::Special(SpecialKind::Dog)
        );
    }
}
