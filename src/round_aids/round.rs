use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card};
use crate::cards::types::{Card, Value};
use crate::round_aids::lay_down::{check_if_lay_down, calculate_score};
use crate::round_aids::lay_down::{optimized_computer_discard};
use crate::util::utils::{prompt_for_number};

pub fn round(player_score: &mut u32, computer_score: &mut u32, hand_size: usize) {
    println!("\nRound Started, Current Hand Size: {}", hand_size);

    println!("Current Scores:\nPlayer - {}\nComputer - {}", player_score, computer_score);

    let mut discard_pile: Vec<Card> = Vec::new();

    let mut deck = create_deck();
    deck = shuffle_deck(deck);

    println!("Deck is Shuffled & Ready, Dealing Cards\n");

    let mut player_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());
    let mut computer_hand = draw_hand(&mut deck, hand_size.try_into().unwrap());

    let discard_pile_start = draw_card(&mut deck);
    discard_pile.push(discard_pile_start);

    let mut pre_lay_down: bool = true;

    let mut player_down_first = false;

    while pre_lay_down {
        if player_turn(&mut player_hand, &mut deck, &mut discard_pile, false) {
            pre_lay_down = false;
            player_down_first = true;
            println!("Player Was Able to Lay Down Cards");
            break;
        }

        if computer_turn(&mut computer_hand, &mut deck, &mut discard_pile) {
            pre_lay_down = false;
            println!("Computer Was Able to lay Down Following Cards");
            display_cards(&computer_hand);
            break;
        }
    }

    if player_down_first {
        if !computer_turn(&mut computer_hand, &mut deck, &mut discard_pile)  {
            let final_computer_score = calculate_score(&computer_hand);
            println!("Final Hand of Computer, Totaling {} Pts", final_computer_score);
            display_cards(&computer_hand);
            *computer_score = *computer_score + final_computer_score;
        } else {
            let final_computer_score = calculate_score(&computer_hand);
            println!("Final Hand of Computer, Totaling {} Pts", final_computer_score);
            display_cards(&computer_hand);
        }
    } else {
        if !player_turn(&mut player_hand, &mut deck, &mut discard_pile, true) {
            *player_score = *player_score + calculate_score(&player_hand);
        }
    }
}

fn player_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>, final_turn: bool) -> bool{
    // Draw Card, selecting between discard and draw pile
    // Display Options to Discard
    // Allow for Checking of Lay Down Capability
    // Return True if still going, False if Laid Down

    if final_turn {
        println!("\n\nComputer Has Laid Down, Final Turn of Round:")
    } else {
        println!("Player Turn Started:");
    }
    println!("The Top of the Discard Pile is:");

    // Extract the top card from discard_pile and clone it
    if let Some(top_card_raw) = discard_pile.last_mut() {
        let top_card: Vec<Card> = vec![top_card_raw.clone()];
        display_cards(&top_card);
    } else {
        println!("Discard pile is empty!");
    }

    println!("Your Current Hand Is:");
    display_cards(hand);

    let draw_instructions_str = "Enter the corresponding number to the pile you want to draw from\n1 - Draw from Deck\n2 - Draw from Top of Discard Pile";
    let draw_decision = prompt_for_number(draw_instructions_str, 1, 2);

    if draw_decision == 1 {
        let card = draw_card(deck);
        hand.push(card);
    }
    else {
        let card = draw_card(discard_pile);
        hand.push(card);
    }

    println!("Your Hand Current is:");

    display_hand(hand);

    if !final_turn{

        let discard_instructions_str = "Enter 0 if you are ready to try to lay down your hand, or...\nEnter the number representing the position of the card that you wish to discard.";
        let discard_index = prompt_for_number(discard_instructions_str, 0, hand.len().try_into().unwrap());

        if discard_index > 0 {
            let discarded_card = hand.remove((discard_index - 1).into());
            discard_card(discard_pile, discarded_card);
            println!("Discarded a card");
        }   
        else {

            let secondary_discard_str = "Enter the number representing the position of the card that you wish to discard.";
            let secondary_discard_index = prompt_for_number(secondary_discard_str, 1, hand.len().try_into().unwrap());

            let discarded_card = hand.remove((secondary_discard_index - 1).into());
            discard_card(discard_pile, discarded_card);
            println!("Discarded a card");

            let lay_down = check_if_lay_down(hand, true);

            return lay_down;
        }
    
    } else {
        let discard_instructions_str = "Enter the number representing the position of the card that you wish to discard as your final discard for the turn.";
        let discard_index = prompt_for_number(discard_instructions_str, 1, hand.len().try_into().unwrap());

        let discarded_card = hand.remove((discard_index - 1).into());
        discard_card(discard_pile, discarded_card);
        println!("Discarded a card");

        let lay_down = check_if_lay_down(hand, true);

        return lay_down;
    }

    return false;
}

fn computer_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) -> bool {

    // Draw a card from the deck
    println!("\nComputer Turn Started, Drawing Card");
    let card = draw_card(deck);
    hand.push(card);

    // Simulate discarding a card
    if !hand.is_empty() {
        let discard_index = optimized_computer_discard(hand);
        let discarded_card = hand.remove(discard_index);
        discard_card(discard_pile, discarded_card); // Pass the owned card
        println!("Discarded a card:\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        // display_cards(&discard_pile, discard_pile.len());
    } else {
        println!("No cards to discard.");
    }

    let lay_down = check_if_lay_down(hand, false);

    // println!("Finishing Turn with Hand:");
    // display_cards(hand, hand.len());

    return lay_down;
}


// Round Structure:

// - Cards delt to player & computer
// - Player draws card
// - Player chooses card to discard, or discards & lays down
// - Computer does the same
// - Repeats until laydown happens
// - Opponent has one more round & lays down what they have
// - Points are tallied up