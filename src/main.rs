use std::io;

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
    println!("1 - Play Five Crowns\n2 - How To Play?\n0 - Exit Game");

    // Declare `input_selection` as a String
    let mut input_selection = String::new();

    // Read input into `input_selection` using stdin
    io::stdin()
        .read_line(&mut input_selection)
        .expect("Failed to read line");

    // Trim and parse the input into a u8
    match input_selection.trim().parse::<u8>() {
        Ok(value) => {
            if value < 3 {
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