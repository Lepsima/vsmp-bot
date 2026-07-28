use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub struct Card {
    value: u8,
    suit: u8,
}

impl Card {
    pub const CODES: &'static [u8] = "A234567890JQK".as_bytes();
    pub const SUITS: &'static [char] = &['♥', '♣', '♦', '♠'];
    pub const TOTAL: usize = Self::CODES.len() * Self::SUITS.len();

    pub fn new<'a>(value: u8, suit: u8) -> Card {
        Card { value, suit }
    }

    pub fn empty<'a>() -> Card {
        Card { value: 0, suit: 0 }
    }

    pub fn get_value(&self) -> u8 {
        let one_plus = self.value + 1;
        one_plus.clamp(1, 10)
    }

    pub fn get_code(&self) -> u8 {
        Card::CODES[self.value as usize]
    }

    pub fn get_suit(&self) -> char {
        Card::SUITS[self.suit as usize]
    }

    pub fn render(cards: &Vec<Card>) -> String {
        let mut lines: [String; 3] = Default::default();

        for card in cards {
            let code = card.get_code() as char;
            let number = if code == '0' { "10" } else { &code.to_string() };

            lines[0] += &format!("┌──{:─>2}", number);
            lines[1] += &format!("│ {} │", card.get_suit());
            lines[2] += &format!("{:─<2}──┘", number);
        }

        format!("```\n{}\n{}\n{}\n```", lines[0], lines[1], lines[2])
    }
}

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn decks(decks: usize) -> Deck {
        let mut cards = Vec::with_capacity(Card::TOTAL * decks);

        for i in 0..Card::TOTAL {
            let code = i % 13;
            let suit = i / 13;

            for _j in 0..decks {
                cards.push(Card::new(code as u8, suit as u8));
            }
        }

        Deck { cards }
    }

    pub fn full() -> Deck {
        Self::decks(1)
    }

    pub fn from(count: usize, other: &mut Deck) -> Deck {
        let cards = (&mut *other).take(count);
        Deck { cards }
    }

    pub fn empty() -> Deck {
        Deck { cards: vec![] }
    }

    pub fn value(&self) -> usize {
        let mut value = 0;
        let mut has_ace = false;

        for card in &self.cards {
            let card_value = card.get_value();
            has_ace |= card_value == 1;
            value += card_value as usize;
        }

        if has_ace && value <= 11 {
            value += 10;
        }

        value
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

    pub fn render_all(&self) -> String {
        Card::render(&self.cards)
    }

    pub fn render_max(&self, max: usize) -> String {
        if self.cards.len() <= max {
            return self.render_all();
        }

        Card::render(&self.peek(max))
    }
}