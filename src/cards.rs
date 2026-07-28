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
        self.value.clamp(1, 10)
    }

    pub fn get_code(&self) -> char {
        Card::CODES[self.value as usize] as char
    }

    pub fn get_suit(&self) -> char {
        Card::SUITS[self.suit as usize]
    }

    pub fn render(cards: &Vec<Card>) -> String {
        let mut lines: [String; 3] = Default::default();

        for card in cards {
            let code = card.get_code();
            let number = if code == '0' { "10" } else { &code.to_string() };

            lines[0] += &format!("┌──{:─>2}", number);
            lines[1] += &format!("│ {} │", card.get_suit());
            lines[2] += &format!("{:─<2}──┘", number);
        }

        format!("```\n{}\n{}\n{}\n```", lines[0], lines[1], lines[2])
    }
}

pub struct Deck {
    pub cards: [Card; Card::TOTAL],
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: [Card; Card::TOTAL] = [Card::empty(); Card::TOTAL];

        for i in 0..Card::TOTAL {
            let code = i % 13;
            let suit = i / 13;
            cards[i] = Card::new(code as u8 , suit as u8);
        }

        Deck { cards }
    }

    pub fn render_all(&self) -> String {
        let mut vec = self.cards.to_vec();
        let mut rng = rand::rng();

        vec.shuffle(&mut rng);
        let coll: Vec<Card> = vec.iter().take(6).map(|c| *c).collect();
        Card::render(&coll)
    }
}