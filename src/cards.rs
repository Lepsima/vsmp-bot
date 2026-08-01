use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use serenity::all::UserId;

pub struct Pack {
    codes: &'static [u8],
    suits: &'static [char]
}

impl Pack {
    pub const EMPTY: Pack = Pack { codes: &[], suits: &[] };

    pub fn blackjack() -> Pack {
        Pack {
            codes: "A234567890JQK".as_bytes(),
            suits: &['♥', '♣', '♦', '♠'],
        }
    }

    pub fn card_count(&self) -> usize {
        self.codes.len() * self.suits.len()
    }

    pub fn get_code(&self, value: u8) -> u8 {
        self.codes[value as usize]
    }

    pub fn get_suit(&self, suit: u8) -> char {
        self.suits[suit as usize]
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    value: u8,
    pub code: u8,
    pub suit: char,
}

impl Card {
    pub fn new(value: u8, suit: u8, pack: &Pack) -> Card {
        Card {
            value,
            code: pack.get_code(value),
            suit: pack.get_suit(suit)
        }
    }

    pub fn empty() -> Card {
        Card { value: 0, code: 0, suit: ' ' }
    }

    pub fn value_blackjack(&self) -> u8 {
        let one_plus = self.value + 1;
        one_plus.clamp(1, 10)
    }

    pub fn value_poker(&self) -> u8 {
        if self.value != 0 {
            self.value + 1
        } else { 14 }
    }

    pub fn value_of(cards: &Vec<Card>, count: usize) -> usize {
        let mut value = 0;
        let mut has_ace = false;
        let mut c = count;

        for card in cards {
            let card_value = card.value_blackjack();
            has_ace |= card_value == 1;
            value += card_value as usize;

            c -= 1;
            if c == 0 { break; }
        }

        if has_ace && value <= 11 {
            value += 10;
        }

        value
    }

    pub fn render(cards: &Vec<Card>, show: usize, value: bool) -> String {
        let mut lines: [String; 3] = Default::default();
        let mut show_count = 0;

        for card in cards {
            let is_hidden = show_count >= show;
            show_count += 1;

            let is_crossed = false;

            if is_crossed {
                lines[0] += "\\───/";
                lines[1] += "│\\ /│";
                lines[2] += "└─V─┘";

            } else if is_hidden {
                lines[0] += "┌───┐";
                lines[1] += "│ ? │";
                lines[2] += "└───┘";

            } else {
                let code = card.code as char;
                let number = if code == '0' { "10" } else { &code.to_string() };

                lines[0] += &format!("┌──{:─>2}", number);
                lines[1] += &format!("│ {} │", card.suit);
                lines[2] += &format!("{:─<2}──┘", number);
            }
        }

        if value {
            lines[1] += &format!(" -> {}", Card::value_of(cards, show));
        }

        format!("```{}\n{}\n{}```", lines[0], lines[1], lines[2])
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
    codes: usize,
    suits: usize
}

impl Deck {
    pub fn decks(decks: usize, pack: &Pack) -> Deck {
        let mut cards = Vec::with_capacity(pack.card_count() * decks);

        for i in 0..pack.card_count() {
            let code = i % 13;
            let suit = i / 13;

            for _j in 0..decks {
                cards.push(Card::new(code as u8, suit as u8, &pack));
            }
        }

        Deck { cards, codes: pack.codes.len(), suits: pack.suits.len() }
    }

    pub fn full(pack: &Pack) -> Deck {
        Self::decks(1, pack)
    }

    pub fn from(count: usize, other: &mut Deck) -> Deck {
        let cards = (&mut *other).take(count);
        Deck { cards, codes: other.codes, suits: other.suits }
    }

    pub fn empty() -> Deck {
        Deck { cards: vec![], codes: 0, suits: 0 }
    }

    pub fn value_for(&self, count: usize) -> usize {
        Card::value_of(&self.cards, count)
    }

    pub fn value(&self) -> usize {
        Card::value_of(&self.cards, usize::MAX)
    }

    pub fn shuffle(&mut self){
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn peek(&self, count: usize) -> Vec<Card> {
        let clamped = count.min(self.cards.len());
        self.cards.iter().take(clamped).cloned().collect()
    }

    pub fn give(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    pub fn take(&mut self, count: usize) -> Vec<Card> {
        let clamped = count.min(self.cards.len());
        self.cards.drain(0..clamped).collect()
    }

    pub fn draw_from(&mut self, count: usize, other: &mut Deck) {
        let mut taken = (&mut *other).take(count);
        self.give(&mut taken);
    }

    pub fn render_all(&self, value: bool) -> String {
        Card::render(&self.cards, self.cards.len(), value)
    }

    pub fn render_some(&self, show: usize, value: bool) -> String {
        Card::render(&self.cards, show, value)
    }

    pub fn render_max(&self, max: usize, show: usize, value: bool) -> String {
        let cards = if self.cards.len() <= max {
            &self.cards
        } else {
            &self.peek(max)
        };

        Card::render(cards, show, value)
    }

    pub fn get_poker_score(&self, other: &Vec<Card>) -> (u8, u8) {
        let mut cards: Vec<&Card> = self.cards.iter().chain(other.iter()).collect();
        cards.sort_unstable_by_key(|s| s.value_poker());

        let has_ace = cards[cards.len() - 1].value_poker() == 14;
        let mut suit_count: HashMap<char, (u8, u8)> = HashMap::new();
        let mut last_equal_suits: HashSet<char> = HashSet::new();

        let mut last_value = 0;
        let mut equal_streak = 1;

        let mut straight_streak = 1;
        let mut straight_suit: char = ' ';
        let mut is_straight_flush: bool = false;

        let mut final_straight_value = 0;
        let mut final_straight_flush = false;

        let mut best_card: u8 = 0;
        let mut pairs: Vec<u8> = Vec::new();
        let mut trio: u8 = 0;
        let mut four: u8 = 0;

        fn is_better(best: (u8, u8), new: (u8, u8)) -> bool {
            new.0 > best.0 || (new.0 == best.0 && new.1 > best.1)
        }

        for card in self.cards.iter().rev() {
            let value = card.value_poker();
            best_card = best_card.max(value);

            // Count suits
            let mut entry = *suit_count.get(&card.suit).unwrap_or(&(0, 0));
            suit_count.insert(card.suit, (entry.0 + 1, entry.1.max(value)));

            if value == last_value {
                equal_streak += 1;
                last_equal_suits.insert(card.suit);

                if equal_streak == 4 {
                    // Four of a kind
                    four = value;

                } else if equal_streak == 2 {
                    // Fullhouse, two pair, one pair
                    pairs.push(last_value);

                } else if trio == 0 && equal_streak == 3 {
                    // Fullhouse / Three of a kind
                    trio = value;
                }
            }
            else {
                // Royal flush, straight flush, straight
                if last_value - 1 == value {
                    straight_streak += 1;

                    if !last_equal_suits.contains(&straight_suit) {
                        is_straight_flush = false;
                    }

                    // Check if an ace could be placed on the end of the straight
                    if has_ace && value == 2 && straight_streak == 4 {
                        final_straight_value = 5;
                        final_straight_flush = is_straight_flush;
                    }

                    // Straight of 5 found, store
                    if straight_streak == 5 {
                        final_straight_value = value + 4;
                        final_straight_flush = is_straight_flush;
                    }
                } else {
                    // Straight lost, reset count
                    is_straight_flush = true;
                    straight_streak = 1;
                    straight_suit = card.suit;
                }

                last_equal_suits.clear();
                equal_streak = 1;
            }

            last_value = value;
        }

        // Royal flush
        if final_straight_value == 14 && final_straight_flush {
            return (9, 0);
        }

        // Straight flush
        if final_straight_value != 0 && final_straight_flush {
            return (8, final_straight_value);
        }

        // Four of a kind
        if four != 0 {
            return (7, four);
        }

        // Fullhouse
        if trio != 0 && !pairs.is_empty() {
            return (6, trio);
        }

        // Flush
        for &count in suit_count.values() {
            if count.0 >= 5 {
                return (5, count.1);
            }
        }

        // Straight
        if final_straight_value != 0 {
            return (4, final_straight_value);
        }

        // Three of a kind
        if trio != 0 {
            return (3, trio);
        }

        // Two pair
        if pairs.len() > 1 {
            return (2, pairs[0].max(pairs[1]))
        }

        // Pair
        if !pairs.is_empty() {
            return (1, pairs[0]);
        }

        (0, best_card)
    }

    /*
    dd9. ace + flush 4 ending in King
    dd8. 5 increasing values of same suit
    dd7. 4 equal values in a row
    dd6. 3 equal values + 2 equal values (any order)
    dd5. 5 cards of the same suit (they are probably not next to each other)
    dd4. 5 increasing values
    dd3. 3 equal values
    dd2. 2 equal values + 2 equal values
    dd1. 2 equal values
    dd0. highest card
     */
}