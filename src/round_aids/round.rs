// Imports functions used in other files
use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card};
use crate::cards::types::{Card};
use crate::round_aids::lay_down::{check_if_lay_down, calculate_score};
use crate::round_aids::lay_down::{optimized_computer_discard};
use crate::util::utils::{prompt_for_number};

// This is the main round function, which takes in both player & computer scores, as well as the hand size for the round
pub fn round(player_score: &mut u32, computer_score: &mut u32, hand_size: usize) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nRound Started, Current Hand Size: {}", hand_size); // Helpful message for reminding about the current hand size, and therefore the round based wild # | Also blanks out the above round from the screen by dumping in line breaks

    println!("Current Scores:\nPlayer - {}\nComputer - {}", player_score, computer_score); // Message to inform about current scores

    let mut discard_pile: Vec<Card> = Vec::new(); // Vector of cards representing the discard pile

    // Logic to create & shuffle a deck of cards which uses a vector to handle the data structure of card
    // The variable size of it really assists representing a deck of cards, which changes in size all the time
    // Additionally, the shuffle feature is made really simple with the vector, since it can use a built in library to do an O(n) shuffle of the deck!
    let mut deck = create_deck();
    deck = shuffle_deck(deck);

    println!("Deck is Shuffled & Ready, Dealing Cards\n"); // Message to show that the deck was successfully constructed

    // Vectors of cards used to represent the player and computer hand
    // Vectors in this case worked well since it allows for simple indexing, as well as removal and addition of cards throughout the round
    let mut player_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());
    let mut computer_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());

    // Puts a single card from the top of the deck into the discard pile to allow for the first player to have a choice of where to draw from
    let discard_pile_start = draw_card(&mut deck);
    discard_pile.push(discard_pile_start);

    let pre_lay_down: bool = true; // Boolean used to hold the while loop in an active state until one of the checks breaks out of it

    let mut player_down_first = false; // Boolean used to track if the player or computer laid down cards first to allow for the other to take one more turn

    // While loop used to repeatedly call the player turn & computer turn functions until one lays down their hand
    while pre_lay_down {

        // Calls the player turn & then handles them laying down if the turn returns true
        if player_turn(&mut player_hand, &mut deck, &mut discard_pile, false) {
            player_down_first = true;
            println!("Player Was Able to Lay Down Cards");
            break; // Break condition used to prevent the computer turn from taking place if the player lays down
        }

        // Calls the computer turn & then handles them laying down if the turn returns true
        if computer_turn(&mut computer_hand, &mut deck, &mut discard_pile) {
            println!("Computer Was Able to lay Down Following Cards");
            display_cards(&computer_hand);
            break; // Break condition used to improve readability by keeping it symmetrical with above player turn logic
        }
    }

    // Handles allowing the player who didn't lay down first to have a single turn more before tallying up their score
    if player_down_first { // Case of if the player was the one to lay down first, signaling the computer to be able to take one final turn
        if !computer_turn(&mut computer_hand, &mut deck, &mut discard_pile)  { // If the computer cannot lay down after one more turn
            let final_computer_score = calculate_score(&computer_hand); // Tallies up the score of the computer
            
            // outputs the final hand of the computer to show to the player
            println!("Final Hand of Computer, Totaling {} Pts", final_computer_score);
            display_cards(&computer_hand);

            *computer_score = *computer_score + final_computer_score; // Adds the score of the computer's hand to the computer's running score
        } else { // Provides the case for when the computer can lay down after one more turn, showing the computer's hand to the player for clarity
            println!("Final Hand of Computer, Totaling 0 Pts");
            display_cards(&computer_hand);
        }
    } else { // Case of if the computer was the one to lay down first, signlaing the player to take one more final turn
        if !player_turn(&mut player_hand, &mut deck, &mut discard_pile, true) {
            *player_score = *player_score + calculate_score(&player_hand);
        }
    }
}

// A Test Round which implements nearly identical logic as above, however it prompts the user for a hand size & doesn't sequence in a multi round game
// It is both useful for testing how the turns run, as well as giving an introductory round for new players
pub fn test_round() {

    // Starts both player and computer scores to zero
    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    // Prompts the user for a number representing one of the valid handsizes of the game & then runs with that size for the rest of the round
    let hand_size_str = "Enter the number of cards you want in your hand\nMin 3\nMax 13";
    let hand_size: usize = prompt_for_number(hand_size_str, 3, 13).into();

    println!("\nRound Started, Current Hand Size: {}", hand_size); // Output message for handsize

    println!("Current Scores:\nPlayer - {}\nComputer - {}", player_score, computer_score); // Output message to show starting scores are zero

    let mut discard_pile: Vec<Card> = Vec::new(); // Vector of cards representing the discard pile

    // Logic to create & shuffle a deck of cards which uses a vector to handle the data structure of card
    // The variable size of it really assists representing a deck of cards, which changes in size all the time
    // Additionally, the shuffle feature is made really simple with the vector, since it can use a built in library to do an O(n) shuffle of the deck!
    let mut deck = create_deck();
    deck = shuffle_deck(deck);

    println!("Deck is Shuffled & Ready, Dealing Cards\n"); // Message to show that the deck was successfully constructed

    // Vectors of cards used to represent the player and computer hand
    // Vectors in this case worked well since it allows for simple indexing, as well as removal and addition of cards throughout the round
    let mut player_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());
    let mut computer_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());

    // Puts a single card from the top of the deck into the discard pile to allow for the first player to have a choice of where to draw from
    let discard_pile_start = draw_card(&mut deck);
    discard_pile.push(discard_pile_start);

    let pre_lay_down: bool = true; // Boolean used to hold the while loop in an active state until one of the checks breaks out of it

    let mut player_down_first = false; // Boolean used to track if the player or computer laid down cards first to allow for the other to take one more turn

    // While loop used to repeatedly call the player turn & computer turn functions until one lays down their hand
    while pre_lay_down {

        // Calls the player turn & then handles them laying down if the turn returns true
        if player_turn(&mut player_hand, &mut deck, &mut discard_pile, false) {
            player_down_first = true;
            println!("Player Was Able to Lay Down Cards");
            break; // Break condition used to prevent the computer turn from taking place if the player lays down
        }

        // Calls the computer turn & then handles them laying down if the turn returns true
        if computer_turn(&mut computer_hand, &mut deck, &mut discard_pile) {
            println!("Computer Was Able to lay Down Following Cards");
            display_cards(&computer_hand);
            break; // Break condition used to improve readability by keeping it symmetrical with above player turn logic
        }
    }

    // Handles allowing the player who didn't lay down first to have a single turn more before tallying up their score
    if player_down_first { // Case of if the player was the one to lay down first, signaling the computer to be able to take one final turn
        if !computer_turn(&mut computer_hand, &mut deck, &mut discard_pile)  { // If the computer cannot lay down after one more turn
            let final_computer_score = calculate_score(&computer_hand); // Tallies up the score of the computer
            
            // outputs the final hand of the computer to show to the player
            println!("Final Hand of Computer, Totaling {} Pts", final_computer_score);
            display_cards(&computer_hand);

            computer_score = computer_score + final_computer_score; // Adds the score of the computer's hand to the computer's running score
        } else { // Provides the case for when the computer can lay down after one more turn, showing the computer's hand to the player for clarity
            println!("Final Hand of Computer, Totaling 0 Pts");
            display_cards(&computer_hand);
        }
    } else { // Case of if the computer was the one to lay down first, signlaing the player to take one more final turn
        if !player_turn(&mut player_hand, &mut deck, &mut discard_pile, true) {
            player_score = player_score + calculate_score(&player_hand);
        }
    }

    println!("Final Scores:\nPlayer - {}\nComputer - {}", player_score, computer_score); // Outputs the final scores to the player to demonstrate the round's end
}

// Lays out the sequence of a player taking their turn & returns a boolean representation of if they have laid down their end, signaling that the end of the round is near
fn player_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>, final_turn: bool) -> bool{

    // Outputs the beginning of turn message to the player based on whether or not it is the final turn of the round
    // This is useful information to signal to the player given that it can influence what risks they may take
    if final_turn { // option for if it is the final turn of the round
        println!("\n\nComputer Has Laid Down, Final Turn of Round:")
    } else { // option for if this isn't the final turn of the round
        println!("Player Turn Started:");
    }

    
    println!("The Top of the Discard Pile is:"); // Beginning of message to user about the top of the discard pile

    // Extract the top card from discard_pile and clone it
    // This cloned card is then used to output to the player what the top of the discard pile is, so that they may choose to draw it
    // Due to the way that rust handles ownership and the likes, it is necessary to handle it this way
    if let Some(top_card_raw) = discard_pile.last_mut() {
        let top_card: Vec<Card> = vec![top_card_raw.clone()];
        display_cards(&top_card);
    } else {
        println!("Discard pile is empty!");
    }

    // Displays current hand by outputting a message and then calling a function that creates a very simple ascii art representation of the cards in the player's hand
    println!("Your Current Hand Is:");
    display_cards(hand);

    // Prompts the user for a numerical choice of what pile they want to discard from.
    // 1 represents drawing from the top of the deck
    // 2 represents drawing from the top of the discard pile
    let draw_instructions_str = "Enter the corresponding number to the pile you want to draw from\n1 - Draw from Deck\n2 - Draw from Top of Discard Pile";
    let draw_decision = prompt_for_number(draw_instructions_str, 1, 2);

    // draws a card from the appropriate pile given the decision made above
    if draw_decision == 1 { // draws from the top of the deck
        let card = draw_card(deck);
        hand.push(card);
    }
    else { // draws from the top of the discard pile
        let card = draw_card(discard_pile);
        hand.push(card);
    }

    // Used to output the hand post draw to then allow for clear information communication to the user
    // This uses the display hand function rather than display cards, since it additionally outputs a position of each card in the hand, starting from 1 and incrementing upwards
    println!("Your Hand Current is:");
    display_hand(hand);

    // Handles what the player can do regarding choosing a card to discard, since it can vary based on if it is or isn't the final round
    if !final_turn{ // If it is a normal turn

        // Prompts the user for an "index" of the card they wish to discard
        // The index of hand positions starts at 1, since 0 is reserved for laying down
        let discard_instructions_str = "Enter 0 if you are ready to try to lay down your hand, or...\nEnter the number representing the position of the card that you wish to discard.";
        let discard_index = prompt_for_number(discard_instructions_str, 0, hand.len().try_into().unwrap());

        // Based on the previous input, either a card is discarded, or an attempt at laying down is made
        if discard_index > 0 { // Standard option where a choice of card is made to discard

            // Pulls the card from the hand & adds it to the discard pile
            let discarded_card = hand.remove((discard_index - 1).into());
            discard_card(discard_pile, discarded_card);
            println!("Discarded a card");
        }   
        else { // Choice for making an attempt to lay down

            // Prompts the user for the index of the card they want to have as a discard before laying down
            let secondary_discard_str = "Enter the number representing the position of the card that you wish to discard.";
            let secondary_discard_index = prompt_for_number(secondary_discard_str, 1, hand.len().try_into().unwrap());

            // Pulls the card from the hand & adds it to the discard pile
            let discarded_card = hand.remove((secondary_discard_index - 1).into());
            discard_card(discard_pile, discarded_card);
            println!("Discarded a card");

            // Boolean represntation of if they can lay down or not
            let lay_down = check_if_lay_down(hand, true);

            return lay_down; // returns lay down based on if they can or can't lay down
        }
    
    } else { // if it is the final turn
        // Prompts the user for an "index" of the card they wish to discard
        // The index of hand positions starts at 1, since 0 is reserved for laying down, though 0 is not present in this case since this is the last round
        let discard_instructions_str = "Enter the number representing the position of the card that you wish to discard as your final discard for the turn.";
        let discard_index = prompt_for_number(discard_instructions_str, 1, hand.len().try_into().unwrap());

        // Pulls the card from the hand & adds it to the discard pile
        let discarded_card = hand.remove((discard_index - 1).into());
        discard_card(discard_pile, discarded_card);
        println!("Discarded a card");

        // Boolean represntation of if they can lay down or not
        let lay_down = check_if_lay_down(hand, true);

        return lay_down; // returns lay down based on if they can or can't lay down
    }

    return false; // Default return statement, that should never be hit, but will allow for the round to continue if it happens to be
}

// Lays out the sequence of a computer player taking its turn & returns a boolean representation of if it has laid down its end, signaling that the end of the round is near
fn computer_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) -> bool {

    println!("\nComputer Turn Started, Drawing Card"); // Signals that the computer turn has started

    // Draws a card from the top of the deck at all times
    let card = draw_card(deck);
    hand.push(card);

    // Discards a card at an index that is generated using the optimized_computer_discard function
    if !hand.is_empty() {
        let discard_index = optimized_computer_discard(hand);
        let discarded_card = hand.remove(discard_index);
        discard_card(discard_pile, discarded_card); // Pass the owned card
        println!("Discarded a card:\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    } else {
        println!("No cards to discard.");
    }

    // Attempts to lay down each turn, with this boolean representing if it can or can't
    let lay_down = check_if_lay_down(hand, false);

    return lay_down; // Returns the boolean to signal whether or not the computer "has laid down its hand"
}