use serenity::all::CreateEmbed;
use crate::cards::{Deck, Pack};
use crate::Color;

pub struct BlackJack {
    deck: Deck,
    player_hand: Deck,
    dealer_hand: Deck,

    player_bet: usize,
    pub is_playing: bool,
}

impl BlackJack {
    pub fn new() -> BlackJack {
        BlackJack {
            is_playing: false,
            player_bet: 0,
            deck: Deck::empty(),
            player_hand: Deck::empty(),
            dealer_hand: Deck::empty()
        }
    }

    pub fn clear(&mut self) {
        *self = BlackJack::new();
    }

    pub fn is_blackjack(deck: &Deck) -> bool {
        deck.value() == 21 && deck.cards.len() == 2
    }

    pub fn complete_bet(&mut self, is_player_winner: bool) -> usize {
        let mut bet = self.player_bet;
        self.player_bet = 0;

        if is_player_winner {
            if BlackJack::is_blackjack(&self.player_hand) {
                bet = (bet as f64 * 1.5) as usize
            }

            // ADD <bet>
        } else {
            // SUBTRACT <bet>
        }

        bet
    }

    pub fn play(&mut self, bet: usize) -> CreateEmbed {

        self.player_bet = bet;
        self.is_playing = false;

        self.deck = Deck::decks(4, &Pack::blackjack());
        self.deck.shuffle();

        self.player_hand = Deck::from(2, &mut self.deck);
        self.dealer_hand = Deck::from(2, &mut self.deck);

        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        let outcome = if player_value == 21 && dealer_value != 21 {
            let bet = self.complete_bet(true);
            &format!("You won {} dolla!", bet)

        } else if dealer_value == 21 && player_value != 21 {
            let bet = self.complete_bet(false);
            &format!("You lost {} dolla!", bet)

        } else if player_value == 21 && player_value == dealer_value {
            "Push!"
        } else {
            self.is_playing = true;
            &format!("Playing with a bet of: {} dolla", bet)
        };

        let color = if self.is_playing {
            Color::GREEN
        } else {
            Color::RED
        };

        CreateEmbed::new()
            .title(outcome)
            .description(self.display_state())
            .color(color)
    }

    pub fn display_state(&self) -> String {
        let show = if self.is_playing { 1 } else { 9999 };
        let dr= &self.dealer_hand.render_some(show, true);
        let pr = &self.player_hand.render_all(true);
        format!("\n{}{}\n", dr, pr)
    }

    pub fn turn(&mut self, action: &str) -> CreateEmbed {
        if !self.is_playing {
            return CreateEmbed::new()
                .title("Error")
                .description("The game has ended")
                .color(Color::RED);
        }

        let mut output = "".to_string();
        let mut is_player_busted = false;

        match action {
            "hit" | "double down" => {
                self.player_hand.draw_from(1, &mut self.deck);
                let player_value = self.player_hand.value();

                self.is_playing = player_value <= 21;
                is_player_busted = 21 < player_value;

                if action == "double down" {
                    self.player_bet *= 2;
                    self.is_playing = false;
                }
            },

            "stand" => {
                self.is_playing = false;
            },

            _ => {
                return CreateEmbed::new()
                    .title("Error")
                    .description("Unknown action")
                    .color(Color::RED);
            }
        }

        if is_player_busted {
            let display = self.display_state();
            let bet = self.complete_bet(false);

            return CreateEmbed::new()
                .title(format!("You busted, lost {} dolla", bet))
                .description(display)
                .color(Color::RED);
        }

        if self.is_playing {
            return CreateEmbed::new()
                .title(format!("Playing with a bet of {} dolla", self.player_bet))
                .description(self.display_state())
                .color(Color::BLUE);
        }

        self.is_playing = false;
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.draw_from(1, &mut self.deck);
        }

        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();
        let color;

        let outcome = if (player_value > dealer_value && player_value <= 21) || dealer_value > 21 {
            color = Color::GREEN;
            let bet = self.complete_bet(true);
            &format!("You won {} dolla!", bet)

        } else if dealer_value > player_value {
            color = Color::RED;
            let bet = self.complete_bet(false);
            &format!("You lost {} dolla!", bet)

        } else {
            color = Color::BLUE;
            "Push, nothing happens."
        };

        CreateEmbed::new()
            .title(outcome)
            .description(self.display_state())
            .color(color)
    }
}