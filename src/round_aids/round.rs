use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card};
use crate::cards::types::{Card, Value};
use crate::round_aids::lay_down::{check_if_lay_down};
use crate::round_aids::lay_down::{optimized_computer_discard};
use crate::util::utils::{prompt_for_number};

pub fn test_round() {
    println!("Test Round Selected, Creating Basic Deck & Shuffling\n");

    let mut deck = create_deck();
    deck = shuffle_deck(deck);


    println!("Deck is Shuffled & Ready, Drawing Hand\n");

    let mut hand = draw_hand(&mut deck, 3);

    println!("Drew Following Hand\n");

    display_cards(&hand, 3);

    println!("Drawing a card test:");

    hand.push(draw_card(&mut deck));

    println!("Card drawn makes hand into the following:\n");

    display_cards(&hand, 4);
}   

pub fn debug_test_round() {
    println!("Debug Test Round Selected, Creating Basic Deck & Shuffling\n");

    let mut discard_pile: Vec<Card> = Vec::new();

    let mut deck = create_deck();
    deck = shuffle_deck(deck);

    println!("Deck is Shuffled & Ready, Dealing Cards\n");

    let mut player_hand = draw_hand(&mut deck, 7);
    let mut computer_hand = draw_hand(&mut deck, 7);

    let discard_pile_start = draw_card(&mut deck);
    discard_pile.push(discard_pile_start);

    println!("Drew Following Hand\n");

    display_cards(&player_hand, 7);

    // println!("Computer has Following Hand\n");

    // display_cards(&computer_hand, 3);

    let mut pre_lay_down: bool = true;
    let mut test_turn_limit = 10;

    while pre_lay_down {
        if player_turn(&mut player_hand, &mut deck, &mut discard_pile) {
            pre_lay_down = false;
        }

        if computer_turn(&mut computer_hand, &mut deck, &mut discard_pile) {
            pre_lay_down = false;
        }

        test_turn_limit -= 1;

        if test_turn_limit == 0 {
            pre_lay_down = false;
        }
    }
}   

fn player_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) -> bool{
    // Draw Card, selecting between discard and draw pile
    // Display Options to Discard
    // Allow for Checking of Lay Down Capability
    // Return True if still going, False if Laid Down

    println!("Player Turn Started:");
    println!("The Top of the Discard Pile is:");

    // Extract the top card from discard_pile and clone it
    if let Some(top_card_raw) = discard_pile.last_mut() {
        let top_card: Vec<Card> = vec![top_card_raw.clone()];
        display_cards(&top_card, 1);
    } else {
        println!("Discard pile is empty!");
    }

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

    display_hand(hand, 7);

    let discard_instructions_str = "Enter 0 if you are ready to try to lay down your hand, or...\nEnter the number representing the position of the card that you wish to discard.";
    let discard_index = prompt_for_number(discard_instructions_str, 0, hand.len().try_into().unwrap());

    if discard_index > 0 {
        let discarded_card = hand.remove((discard_index - 1).into());
        discard_card(discard_pile, discarded_card);
        println!("Discarded a card");
    }   
    else {
        println!("Laying Down Feature is still under development");

        let secondary_discard_str = "Enter the number representing the position of the card that you wish to discard.";
        let secondary_discard_index = prompt_for_number(secondary_discard_str, 1, hand.len().try_into().unwrap());

        let discarded_card = hand.remove((secondary_discard_index - 1).into());
        discard_card(discard_pile, discarded_card);
        println!("Discarded a card");

        let lay_down = check_if_lay_down(hand);

        return lay_down;
    }

    return false;
}

fn computer_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) -> bool {

    // Draw a card from the deck
    println!("Computer Turn Started, Drawing Card");
    let card = draw_card(deck);
    hand.push(card);

    // Display hand after drawing
    // println!("Hand after drawing a card:");
    // display_cards(&hand, hand.len());

    // Simulate discarding a card
    if !hand.is_empty() {
        let discard_index = optimized_computer_discard(hand);
        let discarded_card = hand.remove(discard_index);
        discard_card(discard_pile, discarded_card); // Pass the owned card
        println!("Discarded a card:");
        // display_cards(&discard_pile, discard_pile.len());
    } else {
        println!("No cards to discard.");
    }

    let lay_down = check_if_lay_down(hand);

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