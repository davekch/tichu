use strum::IntoEnumIterator; // iterate over static enum
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
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

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum SpecialKind {
    Dragon,
    Phoenix,
    Dog,
    One,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Special(SpecialKind),
    Regular(RegularKind),
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    None,
}


#[derive(Debug, PartialEq)]
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
