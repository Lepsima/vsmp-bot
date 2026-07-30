use poise::serenity_prelude;
use crate::cards::{Deck, Pack};
use crate::data::ScoreDb;
use crate::Color;
use serenity::all::{CreateEmbed, UserId};
use tokio::sync::MutexGuard;

pub struct BlackJack {
    pub is_playing: bool,
    player_bet: usize,
    user_id: UserId,

    deck: Deck,
    player_hand: Deck,
    dealer_hand: Deck,
}

impl BlackJack {
    pub fn new(user: &UserId) -> BlackJack {
        BlackJack {
            is_playing: false,
            player_bet: 0,
            user_id: user.clone(),
            deck: Deck::empty(),
            player_hand: Deck::empty(),
            dealer_hand: Deck::empty()
        }
    }

    pub fn clear(&mut self) {
        self.is_playing = false;
        self.player_bet = 0;
        self.player_hand = Deck::empty();
        self.dealer_hand = Deck::empty();
    }

    pub fn is_blackjack(deck: &Deck) -> bool {
        deck.value() == 21 && deck.cards.len() == 2
    }

    pub fn complete_bet(&mut self, is_player_winner: bool, score: &mut MutexGuard<ScoreDb>) -> usize {
        let bet = self.player_bet;
        let diff = if !is_player_winner {
            -(bet as i128)
        } else if !BlackJack::is_blackjack(&self.player_hand) {
            bet as i128
        } else {
            (self.player_bet as f64 * 1.5) as i128
        };

        let curr_score = score.get(self.user_id) + diff;
        let _ = score.set(self.user_id, curr_score);
        self.player_bet = 0;
        bet
    }

    pub fn play(&mut self, bet: usize, score: &mut MutexGuard<ScoreDb>) -> CreateEmbed {
        self.player_bet = bet;
        self.is_playing = false;

        let footer = if self.deck.cards.len() < 15 {
            self.deck = Deck::decks(4, &Pack::blackjack());
            self.deck.shuffle();

            "Deck has been re-shuffled!"
        } else { "" };

        self.player_hand = Deck::from(2, &mut self.deck);
        self.dealer_hand = Deck::from(2, &mut self.deck);

        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        let outcome = if player_value == 21 && dealer_value != 21 {
            let bet = self.complete_bet(true, score);
            &format!("You won {} dolla!", bet)

        } else if dealer_value == 21 && player_value != 21 {
            let bet = self.complete_bet(false, score);
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
            .footer(serenity_prelude::CreateEmbedFooter::new(footer))
    }

    pub fn display_state(&self) -> String {
        let show = if self.is_playing { 1 } else { 9999 };
        let dr= &self.dealer_hand.render_some(show, true);
        let pr = &self.player_hand.render_all(true);
        format!("\n{}{}\n", dr, pr)
    }

    pub fn turn(&mut self, action: &str, score: &mut MutexGuard<ScoreDb>) -> CreateEmbed {
        if !self.is_playing {
            return CreateEmbed::new()
                .title("Error")
                .description("The game has ended")
                .color(Color::RED);
        }

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
            let bet = self.complete_bet(false, score);

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
            let bet = self.complete_bet(true, score);
            &format!("You won {} dolla!", bet)

        } else if dealer_value > player_value {
            color = Color::RED;
            let bet = self.complete_bet(false, score);
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