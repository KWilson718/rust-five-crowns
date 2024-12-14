use std::collections::HashMap;
use std::collections::HashSet;

use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card, sort_cards};
use crate::cards::types::{Card, Value, Suit};

// Currently set to false for all time so that the circular round logic can be played without needing to handle the check if lay down function works. 
pub fn check_if_lay_down(hand: &mut Vec<Card>) -> bool {

    let clear_for_lay_down = calculate_score(&hand);

    println!("The Calculate Score Function Returned: {}", clear_for_lay_down);

    if clear_for_lay_down == 0 {
        println!("Successfully Laid Down Cards");
        return true;
    } else {
        println!("Couldn't Lay Down Hand");
        return false;
    }
}

// Discard optimized card (for now just discard the first card in the array to be able to build out turn structure)
pub fn optimized_computer_discard(hand: &mut Vec<Card>) -> usize {
    let mut min_score = u32::MAX; // Initialize to the maximum possible score
    let mut discard_index = 0; // Initialize the index to discard

    for i in 0..hand.len() {
        // Create a temporary hand without the current card
        let mut temp_hand = hand.clone();
        temp_hand.remove(i);

        // Calculate the score of the temporary hand
        let temp_score = calculate_score(&temp_hand);

        // Update the minimum score and discard index if this score is lower
        if temp_score < min_score {
            min_score = temp_score;
            discard_index = i;
        }
    }

    discard_index
}


// Extract wilds from the hand
fn extract_wilds(hand: &mut Vec<Card>, wild_cards: &mut Vec<Card>, std_cards: &mut Vec<Card>, round_value: u8) {
    for (i, card) in hand.iter().enumerate() {
        if (card.value == Value::Wild) || (card.numeric_value == round_value) {
            wild_cards.push(card.clone());
        } else {
            std_cards.push(card.clone());
        }
    }
}

pub fn calculate_score(hand: &Vec<Card>) -> u32 {
    let current_wild_number = hand.len(); // Round's wild card value is based on hand size
    println!("Current Round's Wild Number is: {}", current_wild_number);

    println!("Calculate Score Called with Following Hand: ");
    display_cards(hand, 8);

    let mut value_groups: HashMap<Value, Vec<&Card>> = HashMap::new();
    let mut suit_groups: HashMap<Suit, Vec<&Card>> = HashMap::new();
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

    for card in hand {
        if card.suit == Suit::Wild || card.value == round_wild_card {
            wild_cards.push(card);
        } else {
            suit_groups.entry(card.suit).or_default().push(card);
            value_groups.entry(card.value).or_default().push(card);
        }
    }

    // println!("Number of Wild Cards Detected: {}", wild_cards.len());

    let mut grouped_cards: Vec<&Card> = vec![];

    form_books(&mut value_groups, &mut wild_cards, &mut grouped_cards, &mut used_wild_cards, round_wild_card);
    form_runs(&mut suit_groups, &mut wild_cards, &mut grouped_cards, &mut used_wild_cards, round_wild_card);

    // println!("Grouped Cards After Forming Books and Runs:");
    for card in &grouped_cards {
        card.describe();
    }

    // Create a set of grouped cards to easily check exclusion
    let grouped_card_set: HashSet<_> = grouped_cards.iter().collect();
    let mut used_wild_card_set: HashSet<_> = used_wild_cards.iter().collect();

    // println!("Grouped card set: {:?}", grouped_card_set);
    // println!("Grouped Used Wilds: {:?}", used_wild_card_set);


    // Checks to see if there are any unused final wilds that can be fit into ANY existing group, but were missed by the previous functions
    if (grouped_card_set.len() > 0) && (used_wild_card_set.len() < wild_cards.len()) {
        used_wild_card_set = wild_cards.iter().collect(); // Represents the rest of the wilds getting tossed into random groups to finish out
    }

    
    // Compute the final score, ensuring grouped cards are excluded
    let score: u32 = hand.iter().filter(|card| !grouped_card_set.contains(card) && !used_wild_card_set.contains(card)).map(|card| {
        println!("Card being scored: {:?}", card);
        match card.value {
            Value::Wild => 50,
            v if v == round_wild_card => 20,
            _ => card.numeric_value as u32,
        }
    }).sum();

    // println!("Final calculated score: {}", score);
    score
}


fn form_books<'a>(
    value_groups: &mut HashMap<Value, Vec<&'a Card>>, 
    wild_cards: &mut Vec<&'a Card>, 
    grouped_cards: &mut Vec<&'a Card>, 
    used_wild_cards: &mut Vec<&'a Card>, 
    round_wild_card: Value
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
            } else {
                // Handle the gap with wild cards
                let gap_size = (current_value - prev_value - 1) as usize;
                // println!("Found Gap in Run of Size: {}", gap_size);
                if remaining_wilds.len() >= gap_size {
                    for _ in 0..gap_size {
                        if let Some(wild_card) = remaining_wilds.pop() {
                            current_run.push(wild_card);
                            used_wild_cards.push(wild_card);
                            // println!("Adding wild card to fill gap: {:?}", wild_card);
                        }
                    }
                    current_run.push(*card);
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



// Used locally for debugging related items

pub fn display_cards_debug(deck: &Vec<&Card>, cards_per_row: usize) {
    // Prepare rows for output
    let mut rows = vec!["".to_string(); 7]; // Each card has 6 rows of output

    for (i, card) in deck.iter().enumerate() {
        // Format card components
        let alpha_display = format!("{:>2}", card.alpha_value);
        let suit_display = format!("{:^8}", format!("{:?}", card.suit));

        // Add this card's rows to the output rows
        rows[0].push_str("---------- ");
        rows[1].push_str(&format!("|{}      | ", alpha_display));
        rows[2].push_str("|        | ");
        rows[3].push_str(&format!("|{}| ", suit_display));
        rows[4].push_str("|        | ");
        rows[5].push_str(&format!("|      {}| ", alpha_display));
        rows[6].push_str("---------- ");

        // Print rows if we've reached the limit per row or the end of the deck
        if (i + 1) % cards_per_row == 0 || i + 1 == deck.len() {
            for row in &rows {
                println!("{}", row.trim_end());
            }
            println!(); // Add a blank line between rows of cards
            rows = vec!["".to_string(); 7]; // Reset rows for the next set of cards
        }
    }
}