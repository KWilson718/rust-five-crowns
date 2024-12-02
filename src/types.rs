pub enum Suit {
    Wild,
    Star,
    Spade,
    Club,
    Diamond,
    Heart,
}

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
    suit: Suit,
    value: Value,
    numeric_value: u8,
}

impl Card {
    pub fn new(suit: Suit, value: Value, numeric_value: u8) -> Self{
        Card {suit, value, numeric_value}
    }

    pub fn describe(&self) {

        match self.suit {
            Suit::Wild => println!("This is a Wild Card"),
            Suit::Star => println!("Suit is Star"),
            Suit::Spade => println!("Suit is Spade"),
            Suit::Club => println!("Suit is Club"),
            Suit::Diamond => println!("Suit is Diamond"),
            Suit::Heart => println!("Suit is Heart"),
        }
        
        match self.value{
            Suit::Three => println!("Its Value is Three"),
            Suit::Four => println!("Its Value is Four"),
            Suit::Five => println!("Its Value is Five"),
            Suit::Six => println!("Its Value is Six"),
            Suit::Seven => println!("Its Value is Seven"),
            Suit::Eight => println!("Its Value is Eight"),
            Suit::Nine => println!("Its Value is Nine"),
            Suit::Ten => println!("Its Value is Ten"),
            Suit::Jack => println!("Its Value is Eleven"),
            Suit::Queen => println!("Its Value is Twelve"),
            Suit::King => println!("Its Value is Thirteen"),
            Suit::Wild => println!("It can Represent any Value"),
        }

        println!("The Numeric Value is: {}", self.numeric_value);

    }
}