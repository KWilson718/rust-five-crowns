use std::io;

mod types;
use types::{Suit, Value, Card};

fn main() {

    // The While loop in this instance is used to simulate a potentially multi-game long session of usage, or the ability to read the rules & return to play
    // So long as the exit option isn't hit, then the game will continue looping through other logic and returning to the menu once ready to repeat
    let mut still_playing: bool = true; // Boolean used to dictate to the while loop that the game is still going
    while still_playing {
        println!("Welcome to Command Line Five Crowns"); // Welcome message when menu hit

        // Fetches the selection from the menu options
        let selection_val = menu();

        // Switch case to handle menu entries
        match selection_val{
            0=>still_playing = false,
            1=>println!("\nPlay Option Selected, Launching Game\n"),
            2=>println!("\nHow To Play Option Selected, Loading Rules\n"),
            3=>println!("\nPlay Test Round Selected, Loading Test Round\n"),
            4=>println!("\nCreate Deck Selected, Building Deck\n"),
            5=>{
                println!("\nCreate Test Card Selected, Generating Card\n");
                create_deck();
            },
            _=>println!("\nInvalid Menu Option, {} Is Not A Valid Selection\n", selection_val),
        }
    }

    println!("Exit Option Selected - Have a Great Day!!!\n"); // Exit Message to show that program exited correctly (and wish user a good day)
}

// Recursive Function to get a u8 Integer representing a menu selection
// Recursion chosen to rapidly repeat the request until a valid option is found & returned
// u8 is the Integer type of choice since it is a very small number, and the menu selections make sense to not allow or use negative numbers
fn menu() -> u8 {
    println!("Please Enter an Option Below:");
    println!("1 - Play Five Crowns\n2 - How To Play?\n3 - Play Test Round\n4 - Create Deck\n5 - Create Test Card\n0 - Exit Game");

    // Declare `input_selection` as a String
    let mut input_selection = String::new();

    // Read input into `input_selection` using stdin
    io::stdin()
        .read_line(&mut input_selection)
        .expect("Failed to read line");

    // Trim and parse the input into a u8
    match input_selection.trim().parse::<u8>() {
        Ok(value) => {
            if value < 6 {
                return value;
            }
            else {
                println!("\nInvalid Menu Selection, Please Enter a Number From The List.\n{} Is Not A Valid Option\n", value);
                return menu();
            }
        },
        Err(_) => {
            println!("\nInvalid Input! Please enter a number.\n");
            menu() // Recursively call `menu` to ask for input again until valid option provided
        }
    }
}

fn create_card(suit: Suit, value: Value, numeric_value: u8) -> Card {
    let card = Card::new(suit, value, numeric_value);

    return card;
}

fn create_deck() {
    // Each deck has 6 jokers, and then 2 copies of every other possible card
    // Per the rules, there are two sets of 58 cards, with one of each possible type + three jokers, but it combines to the above for the actual gameplay
    // To simulate a brand new set of decks, the sorted deck will be created by essentially two of the smaller decks being stacked on top of eachother uising nested loops
    // This will then get sent to a shuffle algorithm

    // Vector being used for a variable sized array that will be able to use the pop function to quickly draw a card off of the end. 
    // While it might perform slower than a fixed size array, this will be really useful for simmulating a rapidly changing deck with its dynamic nature. 
    let mut sorted_deck: Vec<Card> = Vec::new();

    // Iterates through creating a deck twice and adding it to the final deck
    for i in 0..2 {
        // Adds in the three wild cards per deck
        for j in 0..3 {
            sorted_deck.push(create_card(Suit::Wild, Value::Wild, 50));
        }
    }

    // Prints out the deck for viewing of the construction
    for card in &sorted_deck {
        card.describe();
    }
}