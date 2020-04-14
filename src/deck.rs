use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator; // iterate over static enum
use strum_macros::EnumIter;

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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Kind {
    Special(SpecialKind),
    Regular(RegularKind),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Card {
    pub kind: Kind,
    pub color: Option<Color>,
    pub rank: i16,
    pub value: i16,
}

impl Card {
    pub fn regular(kind: RegularKind, color: Color) -> Card {
        // returns a new regular card
        let rank = (kind as i16) + 2;
        let mut value = 0;
        match &kind {
            RegularKind::Five => value = 5,
            RegularKind::Ten => value = 10,
            RegularKind::King => value = 10,
            _ => {}
        };
        // return a card
        Card {
            kind: Kind::Regular(kind),
            color: Some(color),
            rank: rank,
            value: value,
        }
    }

    pub fn special(kind: SpecialKind) -> Card {
        // returns a new special card
        let mut rank = 0;
        let mut value = 0;
        match &kind {
            SpecialKind::Phoenix => {
                rank = 0;
                value = -25;
            }
            SpecialKind::Dragon => {
                rank = 14;
                value = 25
            }
            _ => {} // one and dog have rank and value 0
        };
        // return a card
        Card {
            kind: Kind::Special(kind),
            color: None,
            rank: rank,
            value: value,
        }
    }

    pub fn check_eq(&self, other: &Self) -> bool {
        // check if two cards are considered equal, ignoring color
        match self.kind {
            Kind::Special(SpecialKind::Phoenix) => {
                match other.kind {
                    // phoenix can be equal to any regular card
                    Kind::Regular(_) => true,
                    _ => self.kind == other.kind,
                }
            }
            Kind::Regular(_) => match other.kind {
                Kind::Special(SpecialKind::Phoenix) => true,
                _ => self.kind == other.kind,
            },
            _ => self.kind == other.kind,
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
                deck.push(Card::regular(kind, *color));
            }
        }
        // add special cards to deck
        for kind in SpecialKind::iter() {
            deck.push(Card::special(kind));
        }
        Deck { cards: deck }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&self) -> [Vec<&Card>; 4] {
        let mut hands = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for i in 0..self.cards.len() {
            hands[i % 4].push(&self.cards[i])
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
        let unique_deck: Vec<Card> = deck.cards.clone().into_iter().unique().collect();
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
        // deck shouldn't change
        assert_eq!(deck.cards.len(), 52);
        for hand in &hands {
            // check if each player has 13 cards
            assert_eq!(hand.len(), 13);
        }
    }

    #[test]
    fn test_check_eq() {
        // check two equals
        assert!(Card::check_eq(
            &Card::regular(RegularKind::Three, Color::Blue),
            &Card::regular(RegularKind::Three, Color::Black)
        ));
        // check phoenix
        assert!(Card::check_eq(
            &Card::special(SpecialKind::Phoenix),
            &Card::regular(RegularKind::Three, Color::Red)
        ));
        assert!(Card::check_eq(
            &Card::regular(RegularKind::Three, Color::Green),
            &Card::special(SpecialKind::Phoenix)
        ));
        // check inequality between regular kinds
        assert!(!Card::check_eq(
            &Card::regular(RegularKind::Three, Color::Blue),
            &Card::regular(RegularKind::Queen, Color::Blue)
        ));
        // check inequality between regular and special kind
        assert!(!Card::check_eq(
            &Card::special(SpecialKind::One),
            &Card::regular(RegularKind::Queen, Color::Black)
        ));
        // check inequality between two special kinds
        assert!(!Card::check_eq(
            &Card::special(SpecialKind::Phoenix),
            &Card::special(SpecialKind::Dog)
        ));
    }
}
