use crate::deck::{Card, Kind, RegularKind, SpecialKind};
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Copy, Clone)]
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
        0 => None,
        1 => Some(Combination::Singlet),
        2 => {
            if check_all_equal(cards) {
                Some(Combination::Doublet)
            } else {
                None
            }
        }
        3 => {
            if check_all_equal(cards) {
                Some(Combination::Triplet)
            } else {
                None
            }
        }
        4 => {
            if check_bomb(cards) {
                Some(Combination::Bomb)
            } else {
                None
            }
        }
        5 => {
            if check_fullhouse(cards) {
                Some(Combination::FullHouse)
            } else if check_straightflush(cards) {
                Some(Combination::StraightFlush)
            } else if check_straight(cards) {
                Some(Combination::Straight)
            } else {
                None
            }
        }
        _ => {
            if check_stairs(cards) {
                Some(Combination::Stairs)
            } else if check_straightflush(cards) {
                Some(Combination::StraightFlush)
            } else if check_straight(cards) {
                Some(Combination::Straight)
            } else {
                None
            }
        }
    }
}

fn check_all_equal(cards: &[Card]) -> bool {
    // check if all cards are the same according to check_eq
    cards.iter().all(|c| Card::check_eq(&cards[0], &c))
}

fn check_bomb(cards: &[Card]) -> bool {
    // check if all 4 cards are regular and equal
    let allregular = cards.iter().all(|c| match c.kind {
        Kind::Regular(_) => true,
        _ => false,
    });
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
    for i in 0..(cards.len() - 1) {
        // match kind of current card
        match &cards[i].kind {
            Kind::Regular(_) => {
                // match kind of next card
                match &cards[i + 1].kind {
                    Kind::Regular(_) => {
                        if cards[i].rank + 1 != cards[i + 1].rank {
                            return false;
                        }
                    }
                    Kind::Special(SpecialKind::Phoenix) => {}
                    _ => return false,
                }
            }
            Kind::Special(SpecialKind::Phoenix) => {
                // match kind of next kard
                match &cards[i + 1].kind {
                    // phoenix must be followed by normal card
                    Kind::Regular(_) => {}
                    _ => return false,
                }
            }
            Kind::Special(SpecialKind::One) => {
                // can be followed by two and phoenix
                if (cards[i + 1].kind != Kind::Special(SpecialKind::Phoenix))
                    && (cards[i + 1].kind != Kind::Regular(RegularKind::Two))
                {
                    return false;
                }
            }
            _ => return false,
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
    if cards.len() % 2 != 0 {
        return false;
    }
    // check if first two elements are the same
    if !Card::check_eq(&cards[0], &cards[1]) {
        return false;
    }

    let mut straight1: Vec<Card> = Vec::new();
    let mut straight2: Vec<Card> = Vec::new();
    for card in cards.iter().step_by(2) {
        straight1.push(*card);
    }
    for card in cards[1..].iter().step_by(2) {
        straight2.push(*card);
    }
    check_straight(&straight1) && check_straight(&straight2)
}

#[derive(Debug)]
pub struct Trick {
    // implements the combination of cards that is going to be played
    // this may be a valid combination or not (tricks of invalid combinations
    // may not be played)
    pub combination: Option<Combination>,
    cards: Vec<Card>, // it must be possible to add and remove cards
}

impl Trick {
    pub fn new() -> Trick {
        Trick {
            combination: None,
            cards: Vec::new(),
        }
    }

    pub fn push(&mut self, element: Card) {
        self.cards.push(element);
        self.combination = find_combination(&self.cards);
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

    pub fn empty(&mut self) -> Trick {
        // empty all cards from this trick and return an owned clone
        let combination = self.combination;
        let cards = self.cards.to_vec();
        self.combination = None;
        self.cards = Vec::new();
        Trick {
            combination: combination,
            cards: cards,
        }
    }

    pub fn points(&self) -> i16 {
        self.cards.iter().fold(0, |acc, c| acc + c.value)
    }

    pub fn is_valid(&self) -> bool {
        self.combination != None
    }

    fn total_rank(&self) -> i16 {
        // compute the sum of all ranks
        // if there is no phoenix, this is easy enough
        if Trick::no_phoenix(&self.cards) {
            return self.cards.iter().fold(0, |acc, c| acc + c.rank);
        } else {
            // rank of the phoenix depends on the kind of combination
            if self.combination == Some(Combination::FullHouse) {
                // split cases between 2 + 3 fullhouses and 3 + 2
                let first_two = check_all_equal(&self.cards[0..2]);
                let last_three = check_all_equal(&self.cards[2..5]);
                if first_two && last_three {
                    // find the phoenix
                    // if the phoenix is in the last three, its rank is ambiguous
                    // in this case, take the phoenix to be the higher ranked card
                    if Trick::no_phoenix(&self.cards[0..2]) {
                        let doublet_rank = 2 * min(
                            self.cards[0].rank,
                            Trick::find_nonphoenix_rank(&self.cards[2..5]),
                        );
                        let triplet_rank = 3 * max(
                            self.cards[0].rank,
                            Trick::find_nonphoenix_rank(&self.cards[2..5]),
                        );
                        return doublet_rank + triplet_rank;
                    } else {
                        // in the other case the phoenix forms the doublet
                        return 2 * Trick::find_nonphoenix_rank(&self.cards[0..2])
                            + 3 * self.cards[2].rank;
                    }
                } else {
                    // the fullhouse is 3 + 2
                    // find phoenix again
                    if Trick::no_phoenix(&self.cards[0..3]) {
                        // the phoenix builds the doublet
                        return 2 * Trick::find_nonphoenix_rank(&self.cards[3..5])
                            + 3 * self.cards[0].rank;
                    } else {
                        // phoenix is ambiguous again
                        let doublet_rank = 2 * min(
                            Trick::find_nonphoenix_rank(&self.cards[0..3]),
                            self.cards[3].rank,
                        );
                        let triplet_rank = 3 * max(
                            Trick::find_nonphoenix_rank(&self.cards[0..3]),
                            self.cards[3].rank,
                        );
                        return doublet_rank + triplet_rank;
                    }
                }
            } else if self.combination == Some(Combination::Doublet) {
                return 2 * Trick::find_nonphoenix_rank(&self.cards);
            } else if self.combination == Some(Combination::Triplet) {
                return 3 * Trick::find_nonphoenix_rank(&self.cards);
            }
            // don't care about straights and signglets, they are covered in tops without the need of rank
            return 0;
        }
    }

    fn no_phoenix(cards: &[Card]) -> bool {
        // check if there is no phoenix in cards
        return cards
            .iter()
            .all(|c| c.kind != Kind::Special(SpecialKind::Phoenix));
    }

    fn find_nonphoenix_rank(cards: &[Card]) -> i16 {
        // in a set of cards, return the first rank that is not a phoenix
        for card in cards {
            if card.kind != Kind::Special(SpecialKind::Phoenix) {
                return card.rank;
            }
        }
        return 0;
    }

    pub fn tops(&self, other: &Self) -> Option<bool> {
        // check if a trick beats another, returns None if combinations are not compatible
        if self.combination == None || other.combination == None {
            return None;
        }
        let thiscombination = self.combination.unwrap(); // extract the value from Some()
        let othercombination = other.combination.unwrap();
        if thiscombination != Combination::Bomb
            && thiscombination != Combination::StraightFlush
            && thiscombination != othercombination
        {
            return None;
        }
        // from now on either one of the combinations is bomb or flush or the combinations match
        // go through all possibilities
        if thiscombination == Combination::StraightFlush {
            // beats everything other than a flush and only flush if it's higher or longer
            return Some(
                othercombination != Combination::StraightFlush
                    || self.cards.len() > other.cards.len()
                    || self.cards[0].rank > other.cards[0].rank,
            );
        } else if thiscombination == Combination::Bomb {
            // beats everything except flushs and higher bombs
            return Some(
                (othercombination == Combination::Bomb && self.cards[0].rank > other.cards[0].rank)
                    || othercombination != Combination::StraightFlush,
            );
        } else if thiscombination == Combination::Straight {
            // tops if it is longer or higher
            if self.cards.len() > other.cards.len() {
                return Some(true);
            }
            // if they are equally long, compare starting or end rank (avoid phoenix)
            else if self.cards[0].kind == Kind::Special(SpecialKind::Phoenix)
                || other.cards[0].kind == Kind::Special(SpecialKind::Phoenix)
            {
                return Some(
                    self.cards[self.cards.len()].rank > other.cards[other.cards.len()].rank,
                );
            } else {
                return Some(self.cards[0].rank > other.cards[0].rank);
            }
        } else if thiscombination == Combination::Singlet {
            if self.cards[0].kind == Kind::Special(SpecialKind::Dragon) {
                return Some(true);
            } else if self.cards[0].kind == Kind::Special(SpecialKind::Phoenix)
                && other.cards[0].kind != Kind::Special(SpecialKind::Dragon)
            {
                return Some(true);
            } else {
                return Some(self.cards[0].rank > other.cards[0].rank);
            }
        } else {
            // tops if it is higher ranked
            return Some(self.total_rank() > other.total_rank());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Color;

    #[test]
    fn test_find_doublet() {
        let hand = [
            Card::regular(RegularKind::Six, Color::Green),
            Card::special(SpecialKind::Phoenix),
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
            Card::regular(RegularKind::King, Color::Green),
        ];
        assert_eq!(find_combination(&hand), None);
    }

    #[test]
    fn test_trick() {
        let mut trick = Trick::new();
        let king = Card::regular(RegularKind::King, Color::Black);
        let queen = Card::regular(RegularKind::Queen, Color::Red);
        trick.insert(0, queen);
        assert_eq!(trick.combination, Some(Combination::Singlet));
        trick.insert(1, king);
        assert_eq!(trick.combination, None);
        let king2 = trick.remove(1);
        assert_eq!(king, king2);
        let phoenix = Card::special(SpecialKind::Phoenix);
        trick.insert(0, phoenix);
        assert_eq!(trick.combination, Some(Combination::Doublet));
    }

    #[test]
    fn test_tops_bomb() {
        let mut bomb = Trick::new();
        let redking = Card::regular(RegularKind::King, Color::Red);
        let blueking = Card::regular(RegularKind::King, Color::Blue);
        let greenking = Card::regular(RegularKind::King, Color::Green);
        let blackking = Card::regular(RegularKind::King, Color::Black);
        bomb.push(redking);
        bomb.push(blueking);
        bomb.push(greenking);
        bomb.push(blackking);
        let mut something = Trick::new();
        let blueten = Card::regular(RegularKind::Ten, Color::Blue);
        let greenten = Card::regular(RegularKind::Ten, Color::Green);
        something.push(blueten);
        something.push(greenten);
        assert_eq!(bomb.tops(&something), Some(true));
        assert_eq!(something.tops(&bomb), None); // not compatible
    }

    #[test]
    fn test_tops_doublet() {
        let mut trick1 = Trick::new();
        let blackfive = Card::regular(RegularKind::Five, Color::Black);
        let bluefive = Card::regular(RegularKind::Five, Color::Blue);
        trick1.push(blackfive);
        trick1.push(bluefive);
        let mut trick2 = Trick::new();
        let redten = Card::regular(RegularKind::Ten, Color::Red);
        let blueten = Card::regular(RegularKind::Ten, Color::Blue);
        trick2.push(redten);
        trick2.push(blueten);
        assert_eq!(trick2.tops(&trick1), Some(true));
        assert_eq!(trick1.tops(&trick2), Some(false));
        // check with phoenix
        let mut trick3 = Trick::new();
        let redten = Card::regular(RegularKind::Ten, Color::Red);
        let phoenix = Card::special(SpecialKind::Phoenix);
        trick3.push(redten);
        trick3.push(phoenix);
        assert_eq!(trick3.tops(&trick1), Some(true));
    }

    #[test]
    fn test_tops_fullhouse() {
        // construct a regular fullhouse
        let mut trick1 = Trick::new();
        let bluesix = Card::regular(RegularKind::Six, Color::Blue);
        let greensix = Card::regular(RegularKind::Six, Color::Green);
        let bluetwo = Card::regular(RegularKind::Two, Color::Blue);
        let blacktwo = Card::regular(RegularKind::Two, Color::Black);
        let greentwo = Card::regular(RegularKind::Two, Color::Green);
        trick1.push(bluesix);
        trick1.push(greensix);
        trick1.push(bluetwo);
        trick1.push(blacktwo);
        trick1.push(greentwo);
        println!("trick1 rank {}", trick1.total_rank());
        // construct a bigger fullhouse with phoenix
        let mut trick2 = Trick::new();
        let bluesix = Card::regular(RegularKind::Six, Color::Blue);
        let greensix = Card::regular(RegularKind::Six, Color::Green);
        let phoenix = Card::special(SpecialKind::Phoenix);
        let blacktwo = Card::regular(RegularKind::Two, Color::Black);
        let greentwo = Card::regular(RegularKind::Two, Color::Green);
        trick2.push(bluesix);
        trick2.push(greensix);
        trick2.push(phoenix);
        trick2.push(blacktwo);
        trick2.push(greentwo);
        println!("trick2 rank {}", trick2.total_rank());
        assert_eq!(trick2.tops(&trick1), Some(true));
    }
}
