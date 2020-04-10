mod deck;

use deck::{
    Kind,
    RegularKind,
    SpecialKind,
    Color,
    Card,
};

fn main() {
    let card = Card::new(Kind::Special(SpecialKind::Phoenix), Color::Green);
    println!("{:?}", card);
}
