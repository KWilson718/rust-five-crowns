use rand::thread_rng;
use rand::seq::SliceRandom;

use super::types::{Suit, Value, Card};

pub fn create_card(suit: Suit, value: Value, numeric_value: u8) -> Card {
    let alpha_value: String;

    alpha_value = match numeric_value {
        3 => "3".to_string(),
        4 => "4".to_string(),
        5 => "5".to_string(),
        6 => "6".to_string(),
        7 => "7".to_string(),
        8 => "8".to_string(),
        9 => "9".to_string(),
        10 => "10".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        50 => "W".to_string(),
        _ => "Invalid".to_string(), // Handle unexpected numeric values gracefully
    };

    Card::new(suit, value, numeric_value, &alpha_value)
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

pub fn display_cards(deck: &Vec<Card>) {

    let mut cards_per_row: usize;

    if deck.len() < 7 {
        cards_per_row = 6;
    } else if deck.len() < 15 {
        cards_per_row = deck.len() / 2;
        if deck.len() % 2 == 1 {
            cards_per_row += 1;
        }
    } else {
        cards_per_row = 5;
    }

    // Prepare rows for output
    let mut rows = vec!["".to_string(); 7]; // Each card has 6 rows of output

    for (i, card) in deck.iter().enumerate() {
        // Format card components
        let alpha_display = format!("{:>2}", card.alpha_value);
        let suit_display = format!("{:^8}", format!("{:?}", card.suit));

        // Add this card's rows to the output rows
        rows[0].push_str("---------- ");
        rows[1].push_str(&format!("|{}      | ", alpha_display));
        rows[2].push_str("|        | ");
        rows[3].push_str(&format!("|{}| ", suit_display));
        rows[4].push_str("|        | ");
        rows[5].push_str(&format!("|      {}| ", alpha_display));
        rows[6].push_str("---------- ");

        // Print rows if we've reached the limit per row or the end of the deck
        if (i + 1) % cards_per_row == 0 || i + 1 == deck.len() {
            for row in &rows {
                println!("{}", row.trim_end());
            }
            println!(); // Add a blank line between rows of cards
            rows = vec!["".to_string(); 7]; // Reset rows for the next set of cards
        }
    }
}

pub fn display_hand(deck: &Vec<Card>) {

    let mut cards_per_row: usize;

    if deck.len() < 7 {
        cards_per_row = 6;
    } else if deck.len() < 15 {
        cards_per_row = deck.len() / 2;
        if deck.len() % 2 == 1 {
            cards_per_row += 1;
        }
    } else {
        cards_per_row = 5;
    }

    // Prepare rows for output
    let mut rows = vec!["".to_string(); 8]; // Each card has 6 rows of output

    for (i, card) in deck.iter().enumerate() {
        // Format card components
        let alpha_display = format!("{:>2}", card.alpha_value);
        let suit_display = format!("{:^8}", format!("{:?}", card.suit));
        let index_display = format!("{:^10}", i + 1);

        // Add this card's rows to the output rows
        rows[0].push_str("---------- ");
        rows[1].push_str(&format!("|{}      | ", alpha_display));
        rows[2].push_str("|        | ");
        rows[3].push_str(&format!("|{}| ", suit_display));
        rows[4].push_str("|        | ");
        rows[5].push_str(&format!("|      {}| ", alpha_display));
        rows[6].push_str("---------- ");
        rows[7].push_str(&format!("{:<10} ", index_display));

        // Print rows if we've reached the limit per row or the end of the deck
        if (i + 1) % cards_per_row == 0 || i + 1 == deck.len() {
            for row in &rows {
                println!("{}", row.trim_end());
            }
            println!(); // Add a blank line between rows of cards
            rows = vec!["".to_string(); 8]; // Reset rows for the next set of cards
        }
    }
}

pub fn draw_card(deck: &mut Vec<Card>) -> Card {
    // Check if the deck is empty
    if deck.is_empty() {
        panic!("Cannot draw from an empty deck!");
    }

    // Use pop to remove and return the last card from the deck
    let card = deck.pop().expect("Deck should not be empty"); // Safe because we checked for empty above
    return card;
}

pub fn draw_hand(deck: &mut Vec<Card>, hand_size: u8) -> Vec<Card> {
    let mut hand: Vec<Card> = Vec::new();

    // Draw `hand_size` cards
    for _ in 0..hand_size {
        // Draw a card from the deck and push it to the hand
        let card = draw_card(deck);
        hand.push(card);
    }

    return hand; // Return the hand
}

pub fn discard_card(discard_pile: &mut Vec<Card>, card: Card) {
    discard_pile.push(card);
}

pub fn sort_cards(cards: &mut Vec<Card>){
    cards.sort_by(|a, b|{
        format!("{:?}", a.suit).cmp(&format!("{:?}", b.suit)) // Compare `suit` first
            .then_with(|| a.numeric_value.cmp(&b.numeric_value)) // Then compare `numeric_value`
    });
}