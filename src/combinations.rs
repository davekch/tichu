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


pub fn find_combination(cards: &Vec<Card>) -> Option<Combination> {
    match cards.len() {
        1 => Some(Combination::Singlet),
        2 => if check_all_equal(cards) { Some(Combination::Doublet) } else { None },
        3 => if check_all_equal(cards) { Some(Combination::Triplet) } else { None },
        4 => if check_bomb(cards) { Some(Combination::Bomb) } else { None },
        _ => None
    }
}

fn check_all_equal(cards: &Vec<Card>) -> bool {
    // check if all cards are the same according to check_eq
    cards.iter().all(|c| Card::check_eq(&cards[0], &c))
}

fn check_bomb(cards: &Vec<Card>) -> bool {
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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Color;

    #[test]
    fn test_find_doublet() {
        let hand = vec![
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Special(SpecialKind::Phoenix), Color::None)
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Doublet));
    }

    fn test_find_triplet() {
        // check triplet with phoenix
        let hand = vec![
            Card::new(Kind::Special(SpecialKind::Phoenix), Color::None),
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Regular(RegularKind::Six), Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Triplet));
        // check triplet without pheonix
        let hand = vec![
            Card::new(Kind::Regular(RegularKind::Six), Color::Black),
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Regular(RegularKind::Six), Color::Blue),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Triplet));
        // check invalid triplet
        let hand = vec![
            Card::new(Kind::Regular(RegularKind::Six), Color::Black),
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Regular(RegularKind::Seven), Color::Black),
        ];
        assert_eq!(find_combination(&hand), None);
    }

    fn test_find_bomb() {
        // check valid bomb
        let hand = vec![
            Card::new(Kind::Regular(RegularKind::Six), Color::Black),
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Regular(RegularKind::Six), Color::Blue),
            Card::new(Kind::Regular(RegularKind::Six), Color::Red),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Bomb));
        // check invalid bomb
        let hand = vec![
            Card::new(Kind::Special(SpecialKind::Phoenix), Color::None),
            Card::new(Kind::Regular(RegularKind::Six), Color::Green),
            Card::new(Kind::Regular(RegularKind::Six), Color::Blue),
            Card::new(Kind::Regular(RegularKind::Six), Color::Red),
        ];
        assert_eq!(find_combination(&hand), Some(Combination::Bomb));
    }
}
