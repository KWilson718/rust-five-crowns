// Pulls in libraries from collections to implement HashMap & HashSet functions
use std::collections::HashMap;
use std::collections::HashSet;

// Pulls in the capability to interface with the card type
use crate::cards::types::{Card, Value, Suit};

// This function is used to see if a player can lay down and therefore set in progress the end of the round
// The requirement here is that the score of the player's hand needs to be zero, showing that they can turn the full hand into groups of runs or books
pub fn check_if_lay_down(hand: &mut Vec<Card>, is_player: bool) -> bool {

    let clear_for_lay_down = calculate_score(&hand); // Fetches the score from the calculate score function

    // In order to provide clarity to the player, further output information is added when the is_player bool indicates that this is being called by a player
    // When it is a player, it outputs the score, and states if it can or can't lay down, triggering the lay down functionality if it can
    // When it's just the computer, it simply returns a bool based on the condition of if the score is zero, as to prevent giving the player hidden information abt the computer's hand
    if is_player {
        println!("The Calculate Score Function Returned: {}", clear_for_lay_down);

        // Handles whether or not the player can lay down, and returning the result
        if clear_for_lay_down == 0 {
            println!("Successfully Laid Down Cards\n");
            return true;
        } else {
            println!("Couldn't Lay Down Hand\n");
            return false;
        }
    } else {
        return clear_for_lay_down == 0;
    }

    
}

// Discard optimized card function, to provide a selection for which card the computer should discard at the end of its turn
pub fn optimized_computer_discard(hand: &mut Vec<Card>) -> usize {
    let mut min_score = u32::MAX; // Initialize to the maximum possible score, set to the maximum for ease
    let mut discard_index = 0; // Initialize the index to discard

    for i in 0..hand.len() {
        // Create a temporary hand without the current index i card
        let mut temp_hand = hand.clone();
        temp_hand.remove(i);

        // Calculate the score of the temporary hand
        let temp_score = calculate_score(&temp_hand);

        // Update the minimum score and discard index if this score is lower than the current minimum
        if temp_score < min_score {
            min_score = temp_score;
            discard_index = i;
        }
    }

    // return the index that should be used to discard
    return discard_index;
}

// The primary function to a trilogy of functions used to calculate the score of a hand
// Works to group cards into books & runs, using wild cards throughout them to best optimize a hand
// Returns a u32 representation of the score. u32 was used in this case since it needs to be a large number, and negative numbers aren't in the range of potential scores
pub fn calculate_score(hand: &Vec<Card>) -> u32 {
    // Round's wild card value is based on hand size and is stored in this variable
    let current_wild_number = hand.len(); 

    // A HashMap used to store the various groups of cards that will then be tested to form a group for scoring
    let mut value_groups: HashMap<Value, Vec<&Card>> = HashMap::new();
    let mut suit_groups: HashMap<Suit, Vec<&Card>> = HashMap::new();

    // Vectors that will contain the wild cards to demonstrate which ones exist, as well as which ones have been used
    let mut wild_cards: Vec<&Card> = Vec::new(); // Collection to hold wild cards
    let mut used_wild_cards: Vec<&Card> = Vec::new(); // Collection to hold used wild cards

    // Determine the current round's wild card value
    let round_wild_card = match current_wild_number {
        3 => Value::Three,
        4 => Value::Four,
        5 => Value::Five,
        6 => Value::Six,
        7 => Value::Seven,
        8 => Value::Eight,
        9 => Value::Nine,
        10 => Value::Ten,
        11 => Value::Jack,
        12 => Value::Queen,
        13 => Value::King,
        _ => Value::Wild, // This should not happen in a normal game
    };

    // Filters out the wild cards & adds the other cards to the right group based hash map
    for card in hand {
        if card.suit == Suit::Wild || card.value == round_wild_card { // Wild card case
            wild_cards.push(card);
        } else { // Regular card case
            suit_groups.entry(card.suit).or_default().push(card);
            value_groups.entry(card.value).or_default().push(card);
        }
    }

    // Holder vector for collecting which cards have been used in the various groups
    let mut grouped_cards: Vec<&Card> = vec![];

    // Calls functions to collect the book groups & run groups to show correctly existing groups to exclude from tallying points
    form_books(&mut value_groups, &mut wild_cards, &mut grouped_cards, &mut used_wild_cards, round_wild_card);
    form_runs(&mut suit_groups, &mut wild_cards, &mut grouped_cards, &mut used_wild_cards, round_wild_card);

    // Creates a set of grouped cards to easily check exclusion from scoring, as well as one of used wilds for the same purpose
    let grouped_card_set: HashSet<_> = grouped_cards.iter().collect();
    let mut used_wild_card_set: HashSet<_> = used_wild_cards.iter().collect();

    // Checks to see if there are any unused final wilds that can be fit into ANY existing group, but were missed by the previous functions
    if (grouped_card_set.len() > 0) && (wild_cards.len() > 0) {
        used_wild_card_set.extend(wild_cards.iter()); // Represents the rest of the wilds getting tossed into random groups to finish out
    }

    
    // Compute the final score, ensuring grouped cards are excluded
    let score: u32 = hand.iter().filter(|card| !grouped_card_set.contains(card) && !used_wild_card_set.contains(card)).map(|card| { // Only uses cards that aren't fit into groups at all
        match card.value { // Finds the correct value & adds it to the score
            Value::Wild => 50, // For the Joker Wild Cards
            v if v == round_wild_card => 20, // For the round specific wild card value
            _ => card.numeric_value as u32, // For any other card's value
        }
    }).sum();

    return score; // Returns the score
}

/////////////////////////////////////// Documentation Filter Ends Here So Far ///////////////////////////////////////////////////////

// Used to identify book groups
fn form_books<'a>(
    value_groups: &mut HashMap<Value, Vec<&'a Card>>, 
    wild_cards: &mut Vec<&'a Card>, 
    grouped_cards: &mut Vec<&'a Card>, 
    used_wild_cards: &mut Vec<&'a Card>, 
    _round_wild_card: Value
) {
    for (_, group) in value_groups.iter_mut() {
        // println!("Forming books with group: {:?}", group);
        if group.len() + wild_cards.len() >= 3 {
            grouped_cards.extend(group.iter());
            if group.len() < 3 {
                let wilds_needed = 3 - group.len();
                for _ in 0..wilds_needed {
                    if let Some(wild_card) = wild_cards.pop() {
                        grouped_cards.push(wild_card);
                        used_wild_cards.push(wild_card);
                        // println!("Adding wild card to book: {:?}", wild_card);
                    }
                }
            }
        }
        // println!("Grouped cards after forming books: {:?}", grouped_cards);
    }
}


fn form_runs<'a>(
    suit_groups: &mut HashMap<Suit, Vec<&'a Card>>, 
    wild_cards: &mut Vec<&'a Card>, 
    grouped_cards: &mut Vec<&'a Card>, 
    used_wild_cards: &mut Vec<&'a Card>, 
    round_wild_card: Value
) {
    // println!("The number of wilds passed into the form_runs function is: {}", wild_cards.len());

    for (_, group) in suit_groups.iter_mut() {
        group.sort_by_key(|card| card.numeric_value);

        // display_cards_debug(group, 10);

        let mut current_run: Vec<&Card> = Vec::new();
        let mut remaining_wilds = wild_cards.clone();

        for (i, card) in group.iter().enumerate() {
            let current_value = card.numeric_value;
            let prev_value = if i == 0 { current_value } else { current_run.last().unwrap().numeric_value + 1 };

            // println!("Processing card: {:?}", card);
            // println!("Current value: {}, Previous value: {}", current_value, prev_value);

            if current_value == prev_value {
                current_run.push(*card);
            } else if current_value == prev_value + 1 && remaining_wilds.len() >= 1{
                // println!("Found Current Value + 1 as card");
                if let Some(wild_card) = remaining_wilds.pop() {
                    current_run.push(wild_card);
                    used_wild_cards.push(wild_card);
                    // println!("Adding wild card to fill gap: {:?}", wild_card);
                }
                current_run.push(*card);
            } else if current_value > prev_value + 1 {
                let gap_size = (current_value - prev_value - 1) as usize;
                if remaining_wilds.len() >= gap_size {
                    for _ in 0..gap_size {
                        if let Some(wild_card) = remaining_wilds.pop() {
                            current_run.push(wild_card);
                            used_wild_cards.push(wild_card);
                        }
                    }
                    current_run.push(*card);
                } else {
                    // Not enough wilds to fill the gap, finalize the current run if it's valid
                    if current_run.len() + remaining_wilds.len() >= 3 {
                        grouped_cards.extend(&current_run);
                        for card_in_run in &current_run {
                            if card_in_run.suit == Suit::Wild || card_in_run.value == round_wild_card {
                                used_wild_cards.push(*card_in_run);
                            }
                        }
                    }
                    current_run.clear();
                    current_run.push(*card);
                    remaining_wilds = wild_cards.clone();
                }
            } else {
                // Not enough wilds to fill the gap, finalize the current run if it's valid
                if current_run.len() + remaining_wilds.len() >= 3 {
                    // println!("Finalizing run with current run: {:?}", current_run);
                    grouped_cards.extend(&current_run);
                    for card_in_run in &current_run {
                        if card_in_run.suit == Suit::Wild || card_in_run.value == round_wild_card {
                            used_wild_cards.push(*card_in_run);
                        }
                        // println!("Finalizing run - card in run: {:?}", card_in_run);
                    }
                }
                current_run.clear();
                current_run.push(*card);
                remaining_wilds = wild_cards.clone();
            }

            // Finalize run at the end if applicable
            if i == group.len() - 1 && current_run.len() + remaining_wilds.len() >= 3 {
                // println!("Finalizing run at the end with current run: {:?}", current_run);
                grouped_cards.extend(&current_run);
                for card_in_run in &current_run {
                    if card_in_run.suit == Suit::Wild || card_in_run.value == round_wild_card {
                        used_wild_cards.push(*card_in_run);
                    }
                    // println!("Finalizing run at end - card in run: {:?}", card_in_run);
                }
            }
        }

        // println!("Grouped cards after forming runs: {:?}", grouped_cards);
    }
}