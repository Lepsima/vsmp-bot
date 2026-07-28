use crate::cards::Deck;

pub struct BlackJack {
    deck: Deck,
    player_hand: Deck,
    dealer_hand: Deck,

    player_bet: usize,
    pub is_playing: bool,
}

impl BlackJack {
    pub fn new(bet: usize) -> BlackJack {
        let is_playing = true;
        let player_bet = bet;

        let mut deck = Deck::decks(4);
        deck.shuffle();

        let player_hand = Deck::from(2, &mut deck);
        let dealer_hand = Deck::from(2, &mut deck);

        BlackJack {
            is_playing,
            player_bet,
            deck,
            player_hand,
            dealer_hand
        }
    }

    pub fn play(&self) -> String {
        format!("Playing *blackjack* with a bet of: {}€\n{}", self.player_bet, self.display_state())
    }

    pub fn display_state(&self) -> String {
        let dv = &format!("Dealer hand: [{}]", self.dealer_hand.value());
        let dr= &self.dealer_hand.render_max(8);

        let pv = &format!("Your hand: [{}]", self.player_hand.value());
        let pr = &self.player_hand.render_max(8);

        format!("{}\n{}\n{}\n{}\n", dv, dr, pv, pr)
    }

    pub fn check_end(&mut self) -> &str {
        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        if player_value < 21 && dealer_value < 21 {
            return "";
        }

        if (player_value > dealer_value && player_value <= 21) || dealer_value > 21 {
            // ADD self.player_bet
            self.is_playing = false;
            "\n You won!"
        } else if dealer_value > player_value {
            // SUBTRACT self.player_bet
            self.is_playing = false;
            "\n Dealer won!"
        } else {
            self.is_playing = false;
            "\n Push!"
        }
    }

    pub fn turn(&mut self, action: &str) -> String {
        if !self.is_playing {
            return "The game has ended.".to_string();
        }

        let mut output = "".to_string();
        let mut is_player_busted = false;
        let mut is_player_done = false;

        match action {
            "hit" => {
                self.player_hand.draw_from(1, &mut self.deck);
                is_player_busted = 21 < self.player_hand.value();
            },

            "stand" => {
                is_player_done = true;
            },

            "double down" => {
                self.player_hand.draw_from(1, &mut self.deck);
                is_player_busted = 21 < self.player_hand.value();

                self.player_bet *= 2;
                is_player_done = true;
            },

            _ => {
                output += "Unknown action.\n";
            }
        }
        if is_player_busted {
            self.is_playing = false;
            output += &format!("{}\nYou busted, lost {}€", self.display_state(), self.player_bet);
            // SUBTRACT self.player_bet
            return output;
        }

        if !is_player_done {
            output += &format!("{}\nNext move?", self.display_state());
            return output;
        }

        while self.dealer_hand.value() < 17 {
            self.dealer_hand.draw_from(1, &mut self.deck);
        }

        output += &self.display_state();
        output += self.check_end();
        output
    }
}