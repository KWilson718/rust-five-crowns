#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Wild,
    Star,
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Wild,
}

pub struct Card {
    pub suit: Suit,
    pub value: Value,
    pub numeric_value: u8,
    pub alpha_value: String,
}

impl Card {
    pub fn new(suit: Suit, value: Value, numeric_value: u8, alpha_value: &str) -> Self {
        Card {
            suit,
            value,
            numeric_value,
            alpha_value: alpha_value.to_string(),
        }
    }

    pub fn describe(&self) {
        println!("Suit: {:?}", self.suit);
        println!("Value: {:?}", self.value);
        println!("Numeric Value: {}", self.numeric_value);
        println!("Alphabetical Value: {}", self.alpha_value);
    }
}
