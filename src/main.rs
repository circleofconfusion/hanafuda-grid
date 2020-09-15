extern crate hanafuda_deck_rs;

use hanafuda_deck_rs::{HanafudaDeck, HanafudaCard};

fn main() {
    let mut hanafuda_deck = HanafudaDeck::new();
    hanafuda_deck.shuffle();
}
