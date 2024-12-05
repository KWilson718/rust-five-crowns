use std::io;

mod cards;
// use cards::types::{Suit, Value, Card}; Commented out since the types items aren't currently being used
use cards::deck::{create_deck, shuffle_deck};
use crate::cards::types::{Card};

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
                println!("\n---------- Create Test Card Selected, Generating Card ----------\n");
                let sorted_deck = create_deck();

                // Prints out the deck for viewing of the construction
                // for card in &sorted_deck {
                //     card.describe();
                // }

                // Prints out the size of the deck to verify that the right number of cards were added
                println!("The Deck consists of {} Cards\n", sorted_deck.len());

                println!("---------- Shuffling the Deck ----------");

                let shuffled_deck = shuffle_deck(sorted_deck);

                // for card in &shuffled_deck {
                //     card.describe();
                // }

                display_cards(shuffled_deck, 5);
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
                println!("\nInvalid Menu Selection, Please Enter a Number From The List.\n - {} Is Not A Valid Option\n", value);
                return menu();
            }
        },
        Err(_) => {
            println!("\nInvalid Input! Please enter a number.\n");
            menu() // Recursively call `menu` to ask for input again until valid option provided
        }
    }
}

fn display_cards(deck: Vec<Card>, cards_per_row: usize) {
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
