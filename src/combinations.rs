use crate::deck::{
    Card,
    Kind,
    RegularKind,
    SpecialKind,
};

#[derive(Debug, PartialEq)]
pub enum Combination {
    Singlet,
    Doublet,
    Triplet,
    FullHouse,
    Straight,
    Stairs,
    Bomb,
    StraightFlush,
}


pub fn find_combination(cards: &[Card]) -> Option<Combination> {
    match cards.len() {
        1 => Some(Combination::Singlet),
        2 => if check_all_equal(cards) { Some(Combination::Doublet) } else { None },
        3 => if check_all_equal(cards) { Some(Combination::Triplet) } else { None },
        4 => if check_bomb(cards) { Some(Combination::Bomb) } else { None },
        5 => {
            if check_fullhouse(cards) { Some(Combination::FullHouse) }
            else if check_straightflush(cards) { Some(Combination::StraightFlush) }
            else if check_straight(cards) { Some(Combination::Straight) }
            else { None }
        },
        _ => {
            if check_stairs(cards) { Some(Combination::Stairs) }
            else if check_straightflush(cards) { Some(Combination::StraightFlush) }
            else if check_straight(cards) { Some(Combination::Straight) }
            else { None }
        }
    }
}

fn check_all_equal(cards: &[Card]) -> bool {
    // check if all cards are the same according to check_eq
    cards.iter().all(|c| Card::check_eq(&cards[0], &c))
}

fn check_bomb(cards: &[Card]) -> bool {
    // check if all 4 cards are regular and equal
    let allregular = cards.iter().all(
        |c| match c.kind {
            Kind::Regular(_) => true,
            _ => false
        }
    );
    let allequal = check_all_equal(cards);
    allequal && allregular
}

fn check_fullhouse(cards: &[Card]) -> bool {
    let first_two = check_all_equal(&cards[0..2]);
    let last_three = check_all_equal(&cards[2..5]);
    let first_three = check_all_equal(&cards[0..3]);
    let last_two = check_all_equal(&cards[3..5]);
    (first_two && last_three) || (first_three && last_two)
}

fn check_straight(cards: &[Card]) -> bool {
    // check if all cards are consecutive, allowing one and phoenix
    // for every card in cards check if the next card is one rank above.
    // if it is, continue, if it isn't return false.
    // at the end, return true
    for i in 0..(cards.len()-1) {
        // match kind of current card
        match &cards[i].kind {
            Kind::Regular(_) => {
                // match kind of next card
                match &cards[i+1].kind {
                    Kind::Regular(_) => if cards[i].rank + 1 != cards[i+1].rank { return false },
                    Kind::Special(SpecialKind::Phoenix) => {},
                    _ => return false
                }
            },
            Kind::Special(SpecialKind::Phoenix) => {
                // match kind of next kard
                match &cards[i+1].kind {
                    // phoenix must be followed by normal card
                    Kind::Regular(_) => {},
                    _ => return false
                }
            },
            Kind::Special(SpecialKind::One) => {
                // can be followed by two and phoenix
                if (cards[i+1].kind != Kind::Special(SpecialKind::Phoenix))
                    && (cards[i+1].kind != Kind::Regular(RegularKind::Two))
                {
                    return false
                }
            }
            _ => return false
        }
    }
    true
}

fn check_straightflush(cards: &[Card]) -> bool {
    // straight with equal colors (this rules out one and phoenix)
    let allcolors = cards.iter().all(|c| c.color == cards[0].color);
    let isstraight = check_straight(cards);
    isstraight && allcolors
}

fn check_stairs(cards: &[Card]) -> bool {
    // check if cards consists of consecutive pairs
    // split every second card into a new vec, check if two vecs are
    // straights with equal start
    if cards.len() % 2 != 0 { return false }
    // check if first two elements are the same
    if !Card::check_eq(&cards[0], &cards[1]) { return false }

    let mut straight1: Vec<Card> = Vec::new();
    let mut straight2: Vec<Card> = Vec::new();
    for card in cards.iter().step_by(2) {
        straight1.push(*card);
    }
    println!("");
    for card in cards[1..].iter().step_by(2) {
        straight2.push(*card);
    }
    check_straight(&straight1) && check_straight(&straight2)
}


pub struct Trick {
    // implements the combination of cards that is going to be played
    // this may be a valid combination or not (tricks of invalid combinations
    // may not be played)
    pub combination: Option<Combination>,
    cards: Vec<Card>,  // it must be possible to add and remove cards
}

impl Trick {
    pub fn new() -> Trick {
        Trick {
            combination: None,
            cards: Vec::new(),
        }
    }

    pub fn insert(&mut self, i: usize, element: Card) {
        self.cards.insert(i, element);
        self.combination = find_combination(&self.cards);
    }

    pub fn remove(&mut self, i: usize) -> Card {
        let removed = self.cards.remove(i);
        self.combination = find_combination(&self.cards);
        removed
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Color;

    #[test]
    fn test_find_doublet() {
        let hand = vec![
            Card::regular(RegularKind::Six, Color::Green),
            Card::special(SpecialKind::Phoenix)
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Doublet));
    }

    #[test]
    fn test_find_triplet() {
        // check triplet with phoenix
        let hand = [
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::Six, Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Triplet));
        // check triplet without pheonix
        let hand = [
            Card::regular(RegularKind::Six, Color::Black),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::Six, Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Triplet));
        // check invalid triplet
        let hand = [
            Card::regular(RegularKind::Six, Color::Black),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::Seven, Color::Black),
        ];
        assert_eq!(find_combination(&hand), None);
    }

    #[test]
    fn test_find_bomb() {
        // check valid bomb
        let hand = [
            Card::regular(RegularKind::Six, Color::Black),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::Six, Color::Blue),
            Card::regular(RegularKind::Six, Color::Red),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Bomb));
        // check invalid bomb
        let hand = [
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::Six, Color::Blue),
            Card::regular(RegularKind::Six, Color::Red),
        ];
        assert_ne!(find_combination(&hand), Some(Combination::Bomb));
    }

    #[test]
    fn test_find_fullhouse() {
        // test valid fullhouse
        let hand = [
            Card::regular(RegularKind::Six, Color::Black),
            Card::regular(RegularKind::Six, Color::Green),
            Card::regular(RegularKind::King, Color::Green),
            Card::regular(RegularKind::King, Color::Blue),
            Card::regular(RegularKind::King, Color::Red),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::FullHouse));
        let hand = [
            Card::regular(RegularKind::Six, Color::Black),
            Card::regular(RegularKind::Six, Color::Green),
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::King, Color::Blue),
            Card::regular(RegularKind::King, Color::Red),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::FullHouse));
    }

    #[test]
    fn test_find_straight() {
        // test regular straight
        let hand = [
            Card::regular(RegularKind::Three, Color::Blue),
            Card::regular(RegularKind::Four, Color::Blue),
            Card::regular(RegularKind::Five, Color::Red),
            Card::regular(RegularKind::Six, Color::Blue),
            Card::regular(RegularKind::Seven, Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Straight));
        // test special straight
        let hand = [
            Card::special(SpecialKind::One),
            Card::regular(RegularKind::Two, Color::Blue),
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::Four, Color::Blue),
            Card::regular(RegularKind::Five, Color::Blue),
            Card::regular(RegularKind::Six, Color::Blue),
            Card::regular(RegularKind::Seven, Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Straight));
    }

    #[test]
    fn test_find_straightflush() {
        let hand = [
            Card::regular(RegularKind::Three, Color::Blue),
            Card::regular(RegularKind::Four, Color::Blue),
            Card::regular(RegularKind::Five, Color::Blue),
            Card::regular(RegularKind::Six, Color::Blue),
            Card::regular(RegularKind::Seven, Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::StraightFlush));
    }

    #[test]
    fn test_find_stairs() {
        // check valid stair
        let hand = [
            Card::regular(RegularKind::Nine, Color::Black),
            Card::regular(RegularKind::Nine, Color::Green),
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::Ten, Color::Black),
            Card::regular(RegularKind::Jack, Color::Red),
            Card::regular(RegularKind::Jack, Color::Black),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Stairs));
        // check invalid stair
        let hand = [
            Card::regular(RegularKind::Nine, Color::Black),
            Card::regular(RegularKind::Nine, Color::Green),
            Card::special(SpecialKind::Phoenix),
            Card::regular(RegularKind::Ten, Color::Black),
            Card::regular(RegularKind::Jack, Color::Red),
            Card::regular(RegularKind::Jack, Color::Black),
            Card::regular(RegularKind::Queen, Color::Green),
            Card::regular(RegularKind::King, Color::Green)
        ];
        assert_eq!(find_combination(&hand), None);
    }

    #[test]
    fn test_trick() {
        let mut trick = Trick::new();
        trick.insert(0, Card::regular(RegularKind::Queen, Color::Red));
        assert_eq!(trick.combination, Some(Combination::Singlet));
        trick.insert(1, Card::regular(RegularKind::King, Color::Black));
        assert_eq!(trick.combination, None);
        let king = trick.remove(1);
        assert_eq!(king.kind, Kind::Regular(RegularKind::King));
        trick.insert(0, Card::special(SpecialKind::Phoenix));
        assert_eq!(trick.combination, Some(Combination::Doublet));
    }
}
