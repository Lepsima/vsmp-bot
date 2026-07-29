use rand::seq::SliceRandom;

pub struct Pack {
    codes: &'static [u8],
    suits: &'static [char]
}

impl Pack {
    pub fn blackjack() -> Pack {
        Pack {
            codes: "A234567890JQK".as_bytes(),
            suits: &['♥', '♣', '♦', '♠'],
        }
    }

    pub fn card_count(&self) -> usize {
        self.codes.len() * self.suits.len()
    }

    pub fn get_value(&self, value: u8) -> u8 {
        self.codes[value as usize]
    }

    pub fn get_suit(&self, suit: u8) -> char {
        self.suits[suit as usize] as char
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub value: u8,
    pub suit: char,
}

impl Card {
    pub fn new(value: u8, suit: u8, pack: &Pack) -> Card {
        Card {
            value: pack.get_value(value),
            suit: pack.get_suit(suit)
        }
    }

    pub fn empty() -> Card {
        Card { value: 0, suit: ' ' }
    }

    pub fn get_value(&self) -> u8 {
        let one_plus = self.value + 1;
        one_plus.clamp(1, 10)
    }

    pub fn value_of(cards: &Vec<Card>, count: usize) -> usize {
        let mut value = 0;
        let mut has_ace = false;
        let mut c = count;

        for card in cards {
            let card_value = card.get_value();
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
                let code = card.value as char;
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
    pub cards: Vec<Card>
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

        Deck { cards }
    }

    pub fn full(pack: &Pack) -> Deck {
        Self::decks(1, pack)
    }

    pub fn from(count: usize, other: &mut Deck) -> Deck {
        let cards = (&mut *other).take(count);
        Deck { cards }
    }

    pub fn empty() -> Deck {
        Deck { cards: vec![] }
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
}