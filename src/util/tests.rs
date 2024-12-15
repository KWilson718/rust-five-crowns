// Calls to use external functions from different files
use crate::cards::deck::{create_card, shuffle_deck};
use crate::cards::types::{Card, Suit, Value};
use crate::round_aids::lay_down::{calculate_score};

// A funtion to run through a simmulated hand & check the score to be able to output test info. 
pub fn test_scores() {
    let mut test_hand: Vec<Card> = Vec::new(); // The test hand in vector form

    // Adds a simmulated set of cards
    test_hand.push(create_card(Suit::Star, Value::Five, 5));
    test_hand.push(create_card(Suit::Diamond, Value::Five, 5));
    test_hand.push(create_card(Suit::Heart, Value::Five, 5));
    test_hand.push(create_card(Suit::Club, Value::Five, 5));    
    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Spade, Value::Six, 6));
    test_hand.push(create_card(Suit::Spade, Value::Seven, 7));
    test_hand.push(create_card(Suit::Spade, Value::Eight, 8));

    // Finds the score
    let mut test_score = calculate_score(&test_hand);

    // Prints the score
    println!("-------------------------------------Resulting Score is: {}\n", test_score);

    // The rest of this section goes through simmilar calls to test different situations 

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is: {}\n", test_score);

    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));
    test_hand.push(create_card(Suit::Spade, Value::Seven, 7));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Spade, Value::Five, 5));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));
    test_hand.push(create_card(Suit::Heart, Value::Six, 6));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);

    test_hand.truncate(0);

    test_hand.push(create_card(Suit::Star, Value::Nine, 9));
    test_hand.push(create_card(Suit::Club, Value::Nine, 9));
    test_hand.push(create_card(Suit::Star, Value::Four, 4));
    test_hand.push(create_card(Suit::Wild, Value::Wild, 50));

    test_hand = shuffle_deck(test_hand);

    test_score = calculate_score(&test_hand);

    println!("-------------------------------------Resulting Score is {}\n", test_score);
}