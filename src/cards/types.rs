#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Wild,
    Star,
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            suit: self.suit,
            value: self.value,
            numeric_value: self.numeric_value,
            alpha_value: self.alpha_value.clone(), // Clone the String
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit &&
        self.value == other.value &&
        self.numeric_value == other.numeric_value &&
        self.alpha_value == other.alpha_value
    }
}