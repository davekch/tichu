#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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


#[derive(Debug, PartialEq)]
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
