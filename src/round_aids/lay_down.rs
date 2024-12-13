use std::collections::HashMap;

use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card, sort_cards};
use crate::cards::types::{Card, Value, Suit};

// Currently set to false for all time so that the circular round logic can be played without needing to handle the check if lay down function works. 
pub fn check_if_lay_down(hand: &mut Vec<Card>) -> bool {

    println!("Still Working on getting round based Wilds to work, currently only Jokers implemented");

    let clear_for_lay_down = calculate_score(&hand);

    println!("The Calculate Score Function Returned: {}", clear_for_lay_down);

    if(clear_for_lay_down == 0) {
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

fn extract_wilds(hand: &mut Vec<Card>, wild_cards: &mut Vec<Card>, std_cards: &mut Vec<Card>, round_value: u8) {
    for (i, card) in hand.iter().enumerate() {
        if (card.value == Value::Wild) || (card.numeric_value == round_value) {
            wild_cards.push(card.clone());
        }
        else {
            std_cards.push(card.clone());
        }
    }
}

fn form_books<'a>(value_groups: &mut HashMap<Value, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>) { 
    for (_, group) in value_groups { 
        if group.len() + *wilds as usize >= 3 { 
            grouped_cards.extend(group.iter()); 
            
            if group.len() < 3 { 
                *wilds -= (3 - group.len()) as i32; 
            } 
        } 
    } 
}

fn form_runs<'a>(suit_groups: &mut HashMap<Suit, Vec<&'a Card>>, wilds: &mut i32, grouped_cards: &mut Vec<&'a Card>) { 
    for (_, group) in suit_groups { 
        group.sort_by_key(|card| card.value); 
        
        let mut consecutive = 0; 
        let mut last_value = Value::Three; 

        let mut i = 0;
        while i < group.len() { 
            if group[i].value as u8 == last_value as u8 + 1 { 
                consecutive += 1; 
            } else { 
                consecutive = 1; 
            } 
            last_value = group[i].value; 
            if consecutive + *wilds as usize >= 3 { 
                for j in (consecutive - 1) as usize..(consecutive + *wilds as usize) { 
                    if j < group.len() { grouped_cards.push(group[j]); 
                    } else { 
                        *wilds -= 1; 
                    } 
                } 
                consecutive = 0; 
            } 
            i += 1; 
        }
    } 
} 

pub fn calculate_score(hand: &Vec<Card>) -> u32 { 
    println!("Calculate Score Called with Following Hand: ");
    display_cards(hand, 7);

    let mut value_groups: HashMap<Value, Vec<&Card>> = HashMap::new(); 
    let mut suit_groups: HashMap<Suit, Vec<&Card>> = HashMap::new(); 
    
    let mut wilds = 0; for card in hand { 
        if card.suit == Suit::Wild { 
            wilds += 1; 
        } else { 
            suit_groups.entry(card.suit).or_default().push(card); 
        } 
        value_groups.entry(card.value).or_default().push(card); 
    } 
    
    let mut grouped_cards: Vec<&Card> = vec![]; 
    
    form_books(&mut value_groups, &mut wilds, &mut grouped_cards); 
    form_runs(&mut suit_groups, &mut wilds, &mut grouped_cards); 
    
    hand.iter().filter(|&card| !grouped_cards.contains(&card)).map(|card| { 
        match card.value { 
            Value::Wild => 0, _ => card.numeric_value as u32, 
        } 
    }).sum() 
}