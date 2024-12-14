mod cards;
mod round_aids;
mod util;
mod game_aids;

use cards::deck::{create_deck, create_card, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card};
use crate::cards::types::{Card, Suit, Value};
use round_aids::round::{test_round, debug_test_round};
use round_aids::lay_down::{calculate_score};
use util::utils::{prompt_for_number};
use game_aids::game::{game};

// Handles core prompting of player & calling the right function based on the menu's selection in a loop until the session is over
fn main() {

    // The While loop in this instance is used to simulate a potentially multi-game long session of usage, or the ability to read the rules & return to play
    // So long as the exit option isn't hit, then the game will continue looping through other logic and returning to the menu once ready to repeat
    let mut still_playing: bool = true; // Boolean used to dictate to the while loop that the game is still going
    while still_playing {
        println!("Welcome to Command Line Five Crowns"); // Welcome message when menu hit

        let menu_string = "1 - Play Five Crowns\n2 - How To Play?\n3 - Play Test Round\n4 - Play Debug Test Round\n5 - Test Scores\n0 - Exit Game";

        // Fetches the selection from the menu options
        let selection_val = prompt_for_number(menu_string, 0, 5);

        // Switch case to handle menu entries
        match selection_val{
            0=>still_playing = false, // Triggers the exit option of the while loop to get to the end
            1=>game(),
            2=>println!("\nHow To Play Option Selected, Loading Rules\n"),
            3=>test_round(),
            4=>debug_test_round(),
            5=>test_scores(),
            _=>println!("\nInvalid Menu Option, {} Is Not A Valid Selection\n", selection_val),
        }
    }

    println!("Exit Option Selected - Have a Great Day!!!\n"); // Exit Message to show that program exited correctly (and wish user a good day)
}

fn test_scores() {
    let mut test_hand: Vec<Card> = Vec::new();

    test_hand.push(create_card(Suit::Star, Value::Five, 5));
    test_hand.push(create_card(Suit::Diamond, Value::Five, 5));
    test_hand.push(create_card(Suit::Heart, Value::Five, 5));
    test_hand.push(create_card(Suit::Club, Value::Five, 5));    
    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Spade, Value::Six, 6));
    test_hand.push(create_card(Suit::Spade, Value::Seven, 7));
    test_hand.push(create_card(Suit::Spade, Value::Eight, 8));

    let mut test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is: {}\n", test_score);

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is: {}\n", test_score);

    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));
    test_hand.push(create_card(Suit::Spade, Value::Seven, 7));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));
    test_hand.push(create_card(Suit::Heart, Value::Six, 6));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Star, Value::Nine, 9));
    test_hand.push(create_card(Suit::Club, Value::Nine, 9));
    test_hand.push(create_card(Suit::Star, Value::Four, 4));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);
}