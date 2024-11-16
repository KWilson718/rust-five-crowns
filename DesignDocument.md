# Five Crowns from the Command Line:
-----

## Core Mechanics:

- Each Round has a different hand size
- There is a deck of cards 3 - K with five different suits & the right # of cards of each
- Discard Pile (stack) exists
- Top card can be drawn from either deck, or discard pile
- Hand will be shown, can be added to, and subtracted from
- Computer Player can optimize hands given what it is dealt and what it draws

## Play Structure:

### Game:

- Deck is assembled
- Rounds are played
  - Starting hand size is 3 cards, and increases by one for each round till 13
- End of game, points are tied up & player with lowest number of points wins

### Round Structure:

- Dealer is selected
- Deck is regrouped & shuffled
- Hands are delt to players
- Top card of deck is turned over into discard pile
- Player next in line from dealer starts turn & turns go until someone "lays down their hand" which ends the turn cycle
  - Once the lay down process has occurred, everyone gets one more turn, and lays down what they end with
- Points are added up & written down into the scoreboard
- Cards are collected back into center

### Turn Structure

- Player draws a card from either the top of the deck, or the top of the discard pile
- Card is added to hand
- Player sees if they can lay down, and does if they can
- Player discards selected card to discard

### Laying Down

- Laying down can occur if one can assemble groups of cards with their full hand
- If they can, they can lay down, getting zero points for the round
- If they can't, then the game continues

### Groups

- Groups are made of either runs or books
  - books are groups of cards that share the same value, and can have different suits
    - example: a book can consist of a 7 of clubs, a 7 of hearts, and a 7 of spades
  - runs are groups of cards that share a suit, and are in an ascending order
    - example: a run can consist of a 7 of clubs, an 8 of clubs, and a 8 of clubs
- Groups are a minimum of three of cards to be counted as 0 when points are added
- The Joker Card can always count as any card in existance when making a group
- The card paired with the round's hand size (3 cards = 3, 11 cards = J) are also wild for the round, counting as any card in existance when making a group
- If the hand size permits, multiple groups of at least 3 cards can be formed, however each needs to have a minimum of 3, with no maximum number of cards per group present

### Adding Points

- When the end of a round is hit, points are calculated based on cards that cannot be assembled into groups
- The value of cards depends on the following
  - The Joker Card is always worth 50 points
  - The wild card for the round is worth 20 points
  - Each number card is worth its value in points
  - Jacks are worth 11 points
  - Queens are worth 12 points
  - Kings are worth 13 points
- All points of cards not in groups are added up and added to the overall scoreboard, with cards in groups equalling zero

# How it Works
---

## Cards

- Cards will be a custom object which will have both an identifier (name), a suit, and a value associated with them
  - "Two 58 Card Decks"
  - 6 Jokers
  - 2 sets of five suits
    - Range from 3 to King
- They will be able to be stored in arrays & stacks to represend the default deck, a shuffled and playable deck, a hand, and the discard pile

