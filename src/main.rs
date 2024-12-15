mod cards;
mod round_aids;
mod util;
mod game_aids;

// Includes for the various different files' data
use round_aids::round::{test_round};
use util::utils::{prompt_for_number};
use util::tests::{test_scores};
use game_aids::game::{game, short_game};

// Handles core prompting of player & calling the right function based on the menu's selection in a loop until the session is over
fn main() {

    // The While loop in this instance is used to simulate a potentially multi-game long session of usage, or the ability to read the rules & return to play
    // So long as the exit option isn't hit, then the game will continue looping through other logic and returning to the menu once ready to repeat
    let mut still_playing: bool = true; // Boolean used to dictate to the while loop that the game is still going
    while still_playing {
        println!("Welcome to Command Line Five Crowns"); // Welcome message when menu hit

        let menu_string = "1 - Play Five Crowns\n2 - Play Simplified Game\n3 - Play Single Round\n4 - Test Scores (Dev Tool)\n0 - Exit Game";

        // Fetches the selection from the menu options
        let selection_val = prompt_for_number(menu_string, 0, 4);

        // Switch case to handle menu entries
        match selection_val{
            0=>still_playing = false, // Triggers the exit option of the while loop to get to the end
            1=>game(),
            2=>short_game(),
            3=>test_round(),
            4=>test_scores(),
            _=>println!("\nInvalid Menu Option, {} Is Not A Valid Selection\n", selection_val),
        }
    }

    println!("Exit Option Selected - Have a Great Day!!!\n"); // Exit Message to show that program exited correctly (and wish user a good day)
}