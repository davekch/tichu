mod deck;
mod combinations;

use deck::{
    Deck,
};
use combinations::find_combination;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hands = deck.deal();
    for card in &hands[0] {
        println!("{:?}", card);
    }
}
