// imports the ability to use io with the console
use std::io;

// Prompt for Number Function is designed to be able to handle the various requests that occur for the player to enter in a number
// Since this can be used for a menu, as well as for various instances in the turn, it has been abstracted into a function itself
// This recursively calls itself until it can return a valid selection
// u8 is the Integer type of choice since it is a very small number, and can be efficiently passed back and forth
pub fn prompt_for_number(prompt: &str, min: u8, max: u8) -> u8 {
    
    // These print line statements are used to ask the user to give an input of a number, with the second line being used to output a prompt that is provided as a parameter
    println!("Please Enter an Option Below:");
    println!("{}", prompt);
    
    // Declare a String to store the text input by the user
    let mut input_selection = String::new();

    // Reads input into the string created above
    io::stdin()
        .read_line(&mut input_selection)
        .expect("Failed to read line");

    // Trims & parses the input into a u8
    match input_selection.trim().parse::<u8>() {
        Ok(value) => {

            // Once the entry has been converted into a number, checks are made to see if it is a valid output
            if value <= max && value >= min { // If the number is within the range allowed, then it is simply returned
                return value;
            }
            else { // If the number falls outside of the range allowed, then there is an invalid entry message provided, and the function is called recursively to re-print the selections allowed & ask again
                println!("\nInvalid Menu Selection, Please Enter a Number From The List.\n - {} Is Not A Valid Option\n", value);
                return prompt_for_number(prompt, min, max);
            }
        },
        Err(_) => { // This is used to recursively call the function again if the user entered in something other than a number
            println!("\nInvalid Input! Please enter a number.\n");
            prompt_for_number(prompt, min, max) // Recursively call `menu` to ask for input again until valid option provided
        }
    }
}