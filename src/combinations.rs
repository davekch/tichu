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
            else { None }
        }
        _ => None
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
}