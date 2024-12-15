// imports functions from external files
use crate::round_aids::round::{round};

// This function handles the running of a game, with it tallying up the scores, and calling the round function with the correct parameters
// It goes the standard length of Five Crowns, starting with a hand size of 3, and increasing the hand size by 1 each round until the final round of 13 cards in hand
pub fn game() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nGame Started:"); // message displayed to show that a game has started & clears things above it

    // Variables that will hold the scores of the player & computer
    // u32 used in this instance since the scores can get quite large
    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    // this for loop is used to call the round function & feeding in the right score variables, as well as the correct hand size
    for i in 3..14 {
        round(&mut player_score, &mut computer_score, i);
    }

    println!("\nFinal Scores:\nPlayer - {}\nComputer - {}\n", player_score, computer_score); // Final output of scores

    // Handles displaying a message showing who won based upon comparing the final scores
    if player_score == computer_score {
        println!("The Game Ended In A Tie!!!\n\n\n"); // Tie message
    } else if player_score < computer_score {
        println!("You Scored Higher Than The Computer, YOU WIN!!!\n\n\n"); // Win message
    } else {
        println!("The Computer Scored Higher Than You, The Autorities Have Been Notified...\n\n\n"); // Loss message
    }
}

// This function handles the running of a shortened game, with it tallying up the scores, and calling the round function with the correct parameters
// It is a shortened version of Five Crowns, starting with a hand size of 3, and increasing the hand size by 1 each round until a cutoff round of 6 cards in hand
pub fn short_game() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nShort Game Started:"); // message displayed to show that a game has started & clears things above it

    // Variables that will hold the scores of the player & computer
    // u32 used in this instance since the scores can get quite large
    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    // this for loop is used to call the round function & feeding in the right score variables, as well as the correct hand size
    for i in 3..7 {
        round(&mut player_score, &mut computer_score, i);
    }

    println!("\nFinal Scores:\nPlayer - {}\nComputer - {}\n", player_score, computer_score); // Final output of scores

    // Handles displaying a message showing who won based upon comparing the final scores
    if player_score == computer_score {
        println!("The Game Ended In A Tie!!!\n\n\n"); // Tie message
    } else if player_score < computer_score {
        println!("You Scored Higher Than The Computer, YOU WIN!!!\n\n\n"); // Win message
    } else {
        println!("The Computer Scored Higher Than You, The Autorities Have Been Notified...\n\n\n"); // Loss message
    }
}