use crate::cards::deck::{create_deck, shuffle_deck, display_cards, display_hand, draw_hand, draw_card, discard_card, sort_cards};
use crate::cards::types::{Card, Value};

// Currently set to false for all time so that the circular round logic can be played without needing to handle the check if lay down function works. 
pub fn check_if_lay_down(hand: &mut Vec<Card>) -> bool {
    let round_value: u8 = hand.len().try_into().unwrap();
    
    println!("Starting Laydown Process:");

    println!("Splitting Wilds & Standard Cards\nCurrent Round Specific Wild Card is {}.", round_value);

    let mut wild_cards: Vec<Card> = Vec::new();
    let mut std_cards: Vec<Card> = Vec::new();

    extract_wilds(hand, &mut wild_cards, &mut std_cards, round_value);

    println!("Wilds Found:");
    display_cards(&wild_cards, 7);
    println!("Rest of Cards, sorted");
    sort_cards(&mut std_cards);
    display_cards(&std_cards, 7);
    return false;
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

