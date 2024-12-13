use std::collections::HashMap;

use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card, sort_cards};
use crate::cards::types::{Card, Value, Suit};

// Currently set to false for all time so that the circular round logic can be played without needing to handle the check if lay down function works. 
pub fn check_if_lay_down(hand: &mut Vec<Card>) -> bool {

    println!("Still Working on getting round based Wilds to work, currently only Jokers implemented");

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
    return 0;
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
    let mut wilds = 0;
    let mut wild_cards: Vec<Card> = Vec::new(); // Collection to hold wild cards

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

    // Update to handle turning the round's wild into a wild card as well
    for card in hand { 
        if card.suit == Suit::Wild || card.value == round_wild_card { 
            wilds += 1; 
        } else { 
            suit_groups.entry(card.suit).or_default().push(card); 
        }
        value_groups.entry(card.value).or_default().push(card); 
    }

    println!("Number of Wilds Detected: {}", wilds);

    // Pre-create wild cards for later use
    for _ in 0..wilds {
        wild_cards.push(Card {
            suit: Suit::Wild,
            value: Value::Wild,
            numeric_value: 50,
            alpha_value: String::from("Wild"),
        });
    }

    let mut grouped_cards: Vec<&Card> = vec![];
    let mut used_wilds = 0;

    form_books(&mut value_groups, &mut wilds, &mut grouped_cards, &mut used_wilds, &wild_cards, round_wild_card);
    form_runs(&mut suit_groups, &mut wilds, &mut grouped_cards, &mut used_wilds, &wild_cards, round_wild_card);

    println!("Grouped Cards After Forming Books and Runs:");
    for card in &grouped_cards {
        card.describe();
    }

    // Group remaining wilds if possible
    if wilds >= 3 {
        used_wilds += wilds;
        wilds = 0;
    }

    let score: u32 = hand.iter().filter(|card| !grouped_cards.contains(card)).map(|card| {
        match card.value {
            Value::Wild => 50,
            v if v == round_wild_card => 20,
            _ => card.numeric_value as u32,
        }
    }).sum();

    score
}

fn form_books<'a>(value_groups: &mut HashMap<Value, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>, used_wilds: &mut i32, wild_cards: &'a Vec<Card>, round_wild_card: Value) { 
    for (_, group) in value_groups { 
        if group.len() + *wilds as usize >= 3 { 
            grouped_cards.extend(group.iter());
            if group.len() < 3 { 
                let wilds_needed = 3 - group.len() as i32;
                *used_wilds += wilds_needed;
                *wilds -= wilds_needed;
                for i in 0..wilds_needed {
                    grouped_cards.push(&wild_cards[i as usize]);
                }
            }
        }
    }
}

fn form_runs<'a>(suit_groups: &mut HashMap<Suit, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>, used_wilds: &mut i32, wild_cards: &'a Vec<Card>, round_wild_card: Value) {
    for (_, group) in suit_groups {
        group.sort_by_key(|card| card.value);

        let mut run_start = 0;
        let mut wilds_used = 0_usize;

        for i in 0..group.len() {
            let current_value = group[i].value as u8;

            if i == 0 || current_value == (group[i - 1].value as u8) + 1 {
                // Continue the run
            } else if wilds_used < *wilds as usize {
                wilds_used += 1;
                // Continue the run with a wild card
            } else {
                if i - run_start + wilds_used >= 3 {
                    grouped_cards.extend(&group[run_start..i]);
                    // Deduct used wilds
                    for j in 0..wilds_used {
                        *wilds -= 1;
                        *used_wilds += 1;
                        grouped_cards.push(&wild_cards[j as usize]);
                    }
                }
                run_start = i;
                wilds_used = 0;
            }

            if i == group.len() - 1 && i - run_start + 1 + wilds_used >= 3 {
                grouped_cards.extend(&group[run_start..=i]);
                for j in 0..wilds_used {
                    *wilds -= 1;
                    *used_wilds += 1;
                    grouped_cards.push(&wild_cards[j as usize]);
                }
            }
        }
    }
}


// Seemingly working code that doesn't account for the changes to allow for the round specific wild card

// pub fn calculate_score(hand: &Vec<Card>) -> u32 { 
//     let current_wild_number = hand.len();
//     println!("Current Round's Wild Number is: {}", current_wild_number);

//     println!("Calculate Score Called with Following Hand: ");
//     display_cards(hand, 8);

//     let mut value_groups: HashMap<Value, Vec<&Card>> = HashMap::new(); 
//     let mut suit_groups: HashMap<Suit, Vec<&Card>> = HashMap::new(); 
//     let mut wilds = 0;

//     // Update to handle turning the round's wild into a wild card as well
//     for card in hand { 
//         if card.suit == Suit::Wild { 
//             wilds += 1; 
//         } else { 
//             suit_groups.entry(card.suit).or_default().push(card); 
//         }
//         value_groups.entry(card.value).or_default().push(card); 
//     }

//     println!("Number of Wilds Detected: {}", wilds);

//     let mut grouped_cards: Vec<&Card> = vec![];

//     form_books(&mut value_groups, &mut wilds, &mut grouped_cards);
//     form_runs(&mut suit_groups, &mut wilds, &mut grouped_cards);

//     println!("Grouped Cards After Forming Books and Runs:");
//     for card in &grouped_cards {
//         card.describe();
//     }

//     let score: u32 = hand.iter().filter(|card| !grouped_cards.contains(card)).map(|card| {
//         match card.value {
//             _ => card.numeric_value as u32,
//         }
//     }).sum();

//     score
// }

// fn form_books<'a>(value_groups: &mut HashMap<Value, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>) { 
//     for (_, group) in value_groups { 
//         if group.len() + *wilds as usize >= 3 { 
//             grouped_cards.extend(group.iter());
//             if group.len() < 3 { 
//                 *wilds -= (3 - group.len()) as i32;
//             }
//         }
//     } 
// }

// fn form_runs<'a>(suit_groups: &mut HashMap<Suit, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>) {
//     for (_, group) in suit_groups {
//         group.sort_by_key(|card| card.value);

//         let mut run_start = 0;
//         let mut wilds_used = 0_usize;

//         for i in 0..group.len() {
//             let current_value = group[i].value as u8;

//             if i == 0 || current_value == (group[i - 1].value as u8) + 1 {
//                 // Continue the run
//             } else if wilds_used < *wilds as usize {
//                 wilds_used += 1;
//                 // Continue the run with a wild card
//             } else {
//                 if i - run_start + wilds_used >= 3 {
//                     grouped_cards.extend(&group[run_start..i]);
//                     // Deduct used wilds
//                     for _ in 0..wilds_used {
//                         *wilds -= 1;
//                     }
//                 }
//                 run_start = i;
//                 wilds_used = 0;
//             }

//             if i == group.len() - 1 && i - run_start + 1 + wilds_used >= 3 {
//                 grouped_cards.extend(&group[run_start..=i]);
//                 for _ in 0..wilds_used {
//                     *wilds -= 1;
//                 }
//             }
//         }
//     }
// }