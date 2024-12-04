use rand::thread_rng;
use rand::seq::SliceRandom;

use super::types::{Suit, Value, Card};

fn create_card(suit: Suit, value: Value, numeric_value: u8) -> Card {
    let card = Card::new(suit, value, numeric_value);

    return card;
}

pub fn create_deck() -> Vec<Card>{
    // Each deck has 6 jokers, and then 2 copies of every other possible card
    // Per the rules, there are two sets of 58 cards, with one of each possible type + three jokers, but it combines to the above for the actual gameplay
    // To simulate a brand new set of decks, the sorted deck will be created by essentially two of the smaller decks being stacked on top of eachother uising nested loops
    // This will then get sent to a shuffle algorithm

    // Vector being used for a variable sized array that will be able to use the pop function to quickly draw a card off of the end. 
    // While it might perform slower than a fixed size array, this will be really useful for simmulating a rapidly changing deck with its dynamic nature. 
    let mut sorted_deck: Vec<Card> = Vec::new();

    // Iterates through creating a deck twice and adding it to the final deck
    for _ in 0..2 {
        // Adds in the three wild cards per deck
        for _ in 0..3 {
            sorted_deck.push(create_card(Suit::Wild, Value::Wild, 50));
        }

        // Arrays used to store the various values that matter for a deck, which will be iterated through to generate a deck
        // The array structure is used since it's fixed size and will be very efficient to run through
        // Additionally, the tuple is used for the value and the numeric representation to further add efficiency to the data that is hard-coded to create the deck
        let suits = [Suit::Star, Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];
        let values = [
            (Value::Three, 3),
            (Value::Four, 4),
            (Value::Five, 5),
            (Value::Six, 6),
            (Value::Seven, 7),
            (Value::Eight, 8),
            (Value::Nine, 9),
            (Value::Ten, 10),
            (Value::Jack, 11),
            (Value::Queen, 12),
            (Value::King, 13),
        ];

        // This group of nested for loops goes through and generates each different card to then put into the deck.
        for suit in suits {
            for (value, rank) in values.iter() {
                sorted_deck.push(create_card(suit, *value, *rank));
            }
        }    
    }

    return sorted_deck;    
}

pub fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    // Gets a random number generation value based off of the thread_rng() function from the rand dependency
    // This allows for a more random version of a random number
    let mut rng = thread_rng(); 

    // shuffles the deck based on the rand dependency's interactions with the vector using the previously generated random numnber generator
    deck.shuffle(&mut rng);

    return deck;
}