// This enum holds the different options for suits that cards may have
// An enum type was chosen to show an abstraction of data choices
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)] // These are rust language specific derivations that allow for good usage of the enum throughout the program
pub enum Suit {
    Wild,
    Star,
    Spade,
    Club,
    Diamond,
    Heart,
}

// This enum holds the different options for what a card's value may be
// An enum type was chosen to show an abstraction of data choices
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)] // These are rust language specific derivations that allow for good usage of the enum throughout the program
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

// This struct represents a card, with the suit being an enum, and the value being represented across three variables due to how diferent parts of the program use it
// A struct type was the clear choice for how to abstract the data about the card, since it allows for quick & easy reference of different variable types inside of a single collection which then plays well with being put into arrays, hashes, etc...
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // These are rust language specific derivations that allow for good usage of the struct throughout the program
pub struct Card {
    pub suit: Suit,
    pub value: Value,
    pub numeric_value: u8,
    pub alpha_value: String,
}

// This is the constructor for a card, which is necessary to custom define since it turns the alpha_value into a string literal through the .to_string() being used
impl Card {
    pub fn new(suit: Suit, value: Value, numeric_value: u8, alpha_value: &str) -> Self {
        Card {
            suit,
            value,
            numeric_value,
            alpha_value: alpha_value.to_string(),
        }
    }
}