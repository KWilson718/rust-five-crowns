use std::io;

// Prompt for Number Function is designed to be able to handle the various requests that occur for the player to enter in a number
// Since this can be used for a menu, as well as for various instances in the turn, it has been abstracted into a function itself
// This recursively calls itself until it can return a valid selection
// u8 is the Integer type of choice since it is a very small number, and can be efficiently passed back and forth
pub fn prompt_for_number(prompt: &str, min: u8, max: u8) -> u8 {
    println!("Please Enter an Option Below:");
    println!("{}", prompt);
    
    // Declare `input_selection` as a String
    let mut input_selection = String::new();

    // Read input into `input_selection` using stdin
    io::stdin()
        .read_line(&mut input_selection)
        .expect("Failed to read line");

    // Trim and parse the input into a u8
    match input_selection.trim().parse::<u8>() {
        Ok(value) => {
            if value <= max && value >= min {
                return value;
            }
            else {
                println!("\nInvalid Menu Selection, Please Enter a Number From The List.\n - {} Is Not A Valid Option\n", value);
                return prompt_for_number(prompt, min, max);
            }
        },
        Err(_) => {
            println!("\nInvalid Input! Please enter a number.\n");
            prompt_for_number(prompt, min, max) // Recursively call `menu` to ask for input again until valid option provided
        }
    }
}