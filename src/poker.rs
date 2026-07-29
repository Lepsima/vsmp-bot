use crate::cards::{Deck, Pack};

pub struct Poker {
    deck: Deck,
    player_hand: Deck,
    dealer_hand: Deck,

    player_bet: usize,
    pub is_playing: bool,
}

impl Poker {
    pub fn new() -> Poker {
        Poker {
            is_playing: false,
            player_bet: 0,
            deck: Deck::empty(),
            player_hand: Deck::empty(),
            dealer_hand: Deck::empty()
        }
    }

    pub fn clear(&mut self) {
        *self = Poker::new();
    }
}