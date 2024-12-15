mod cards;
mod round_aids;
mod util;
mod game_aids;

// Includes for the various different files' functions
use round_aids::round::{test_round};
use util::utils::{prompt_for_number};
use util::tests::{test_scores};
use game_aids::game::{game, short_game};

// Handles the core functionality of the game through a while loop that allows for multiple games to be played
// Each option calls out a different function to handle different functionality of the game
fn main() {

    // Ch 6, Section 2, Primitive Types - Boolean Values
    // The usage of a bool here is to clearly provide a switch between on and off for the while loop
    let mut still_playing: bool = true; // Boolean used to dictate to the while loop that the game is still going

    // The While loop in this instance is used to simulate a potentially multi-game long session of usage, or the ability to read the rules & return to play
    // So long as the exit option isn't hit, then the game will continue looping through other logic and returning to the menu once ready to repeat
    while still_playing {
        println!("Welcome to Command Line Five Crowns"); // Welcome message when menu hit

        // Ch 6, Section 3, Strings
        // This string is used to send to the prompt for number function in order to show the options that the player can select from
        let menu_string = "1 - Play Five Crowns\n2 - Play Simplified Game\n3 - Play Single Round\n4 - Test Scores (Dev Tool)\n0 - Exit Game";

        // Ch 6, Section 2, Primitive Data Types - Unsigned Int (there doesn't need to be a negative number)
        // Fetches the selection from the user through this generic function which prompts the user for a number
        let selection_val = prompt_for_number(menu_string, 0, 4);

        // Switch case to handle menu entries
        match selection_val{
            0=>still_playing = false, // Triggers the exit option of the while loop to get to the end
            1=>game(), // The core 11 round game
            2=>short_game(), // A shortened game going from hand size 3 to hand size 6
            3=>test_round(), // A test round to show how the game works without getting into a larger multi-round game
            4=>test_scores(), // A debug option used to test the scoring to make sure that it correctly works. This was used many times throughout development.
            _=>println!("\nInvalid Menu Option, {} Is Not A Valid Selection\n", selection_val), // A default case, which should never be possible to hit given that the prompt user for number function works well
        }
    }

    println!("Exit Option Selected - Have a Great Day!!!\n"); // Exit Message to show that program exited correctly (and wish user a good day)
}