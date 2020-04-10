mod deck;

use deck::{
    Deck,
};

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("{:?}", deck.cards[0]);
}
