#[derive(Copy, Clone)]
pub enum Suit {
    Wild,
    Star,
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Copy, Clone)]
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
            Value::Three => println!("Its Value is Three"),
            Value::Four => println!("Its Value is Four"),
            Value::Five => println!("Its Value is Five"),
            Value::Six => println!("Its Value is Six"),
            Value::Seven => println!("Its Value is Seven"),
            Value::Eight => println!("Its Value is Eight"),
            Value::Nine => println!("Its Value is Nine"),
            Value::Ten => println!("Its Value is Ten"),
            Value::Jack => println!("Its Value is Eleven"),
            Value::Queen => println!("Its Value is Twelve"),
            Value::King => println!("Its Value is Thirteen"),
            Value::Wild => println!("It can Represent any Value"),
        }

        println!("The Numeric Value is: {}\n", self.numeric_value);

    }
}