use crate::round_aids::round::{round};

pub fn game() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nGame Started:");

    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    for i in 3..14 {
        round(&mut player_score, &mut computer_score, i);
    }

    println!("\nFinal Scores:\nPlayer - {}\nComputer - {}\n", player_score, computer_score);

    if player_score == computer_score {
        println!("The Game Ended In A Tie!!!\n\n\n");
    } else if player_score < computer_score {
        println!("You Scored Higher Than The Computer, YOU WIN!!!\n\n\n");
    } else {
        println!("The Computer Scored Higher Than You, You Lose...\n\n\n");
    }
}

pub fn short_game() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nShort Game Started:");

    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    for i in 3..7 {
        round(&mut player_score, &mut computer_score, i);
    }

    println!("\nFinal Scores:\nPlayer - {}\nComputer - {}\n", player_score, computer_score);

    if player_score == computer_score {
        println!("The Game Ended In A Tie!!!\n\n\n");
    } else if player_score < computer_score {
        println!("You Scored Higher Than The Computer, YOU WIN!!!\n\n\n");
    } else {
        println!("The Computer Scored Higher Than You, You Lose...\n\n\n");
    }
}