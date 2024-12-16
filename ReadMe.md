#  Command Line **Five Crowns** in Rust

This Project is both a way to learn the language Rust, as well as a final assignment for a class named Concepts in Programming, where each student is to create a command line interpretation of a board game of their choosing. This is created for educational purposes.

## Description of **Five Crowns**, the Game:

Five Crowns is referred to as a Rummy Style Card Game, meaning that it is in the style of matching types of cards together based on value, or a sequence in the same suit. Another extremely popular Rummy styled game is Rummikub. 

The game is played with an alternative deck, which is made of **two** 58 card decks which contain the following each:

- Three Wild Cards
- Five Different Suits of Cards (Heart, Diamond, Spade, Club, and Star) containing
  - 3 - 10
  - Jack
  - Queen
  - King

Since there are two of the smaller decks merged together, the deck contains six wild cards, and two of each of the rest. This totals out to a 116 card deck which leaves a lot of randomness for traditionally one to seven players.

The game consists of multiple rounds, with the hand starting at three cards, and progressively adding one card to the hand size per round until a handsize of thirteen is attained. 

Within a round, one tries to assemble groups of cards to then "Lay Down" everything in their hand (except for one to discard) into groupings of books & runs to win the round. Once one has been able to lay down their hand, the rest of the players take one more final turn each, and then count up their scores for the round to add to the running tally for the game. 

At the end of the game (the end of the round with hand size 13), the player with the **lowest** points **WINS**!!!

### Difference Between The Card Game & This Game

The key difference between this game & the card game is that it is just you, the player, versus the computer. There currently isn't the option to add other players to the game. 

Additionally, in the card game, the player chooses how to lay down cards, while in this game, there is an automation to tallying up points. This takes away a bit of the nuanced control from the player, but is a bit of a necessity for the scope of the game.

## How to Run the Game:

1. Ensure that Rust & Cargo are installed on your system correctly, one way to check is by doing the following:
    ```
    rustc --version
    cargo --version
    ```
   1. If either doesn't have a version, please re-install Rust & make sure that Cargo is installed alongside it. 
   2. Installation varies based on what machine you are using, however details can be found [Here](https://www.rust-lang.org/learn/get-started)
2. Download this application, and navigate in your terminal to the folder that it is downloaded (and unzipped) in
3. Enter the command `cargo build` to compile the program
4. Enter the command `cargo run` to play the game
5. If you happen to want to remove the compiled code, run `cargo clean` to go back to a fresh start

## How to Play the Game:

### Beginning Menu

Upon First Loading into the game, a few options are presented.

1. Play Game (Standard Length Option)
2. Play Simplified Game (A quick four round shortened game)
3. Play Single Round (A single round to both test & learn the game)
4. Test Scores (this is a debug function that I'm leaving in to test scores)
5. Exit Game (actually accessed by entering 0, but in the fifth position)

The game then prompts for an answer from the player, and then operates based off of that

The Menu should look like the following:

![Main Menu](/screenshots/menu-screenshot.png)

### Starting a Game

Depending on the selection, a game can last 11 rounds (standard), 4 rounds (simplified), or just 1 round (single round). No matter which selection, the two forms of games start the same way, and the single round quickly gets there, but first asks the user what hand size they wish to play a round of

![Hand Size Question](/screenshots/hand-size-question.png)

Once any of those selections have been made, the round officially starts.

The game will then go on for a specified amount of rounds, with each round consisting of turns until one of the players can "lay down" their hand, with the other player having one more turn to optimize their hand before calculating their score

### Playing in a Round

A Round starts off with the following display:

![Start of Round](/screenshots/round-start.png)

As noted in the above image, the game displays the following:

- The current hand size
- Scores for both the player and computer
- Top card of the discard pile (starts off at one card in size)
- Current hand
- Options for drawing

The user then selects between drawing the shown top of the discard pile, or drawing from the deck which is not shown. Once a draw decision has been made, the game prints out the player's hand again for them to then decide to discard a card from their hand to return back to handsize:

![Discard Decision](/screenshots/discard-decision.png)

This is when the player's chance to strategize comes into effect... 

The goal of the player is to form runs and books. 

- Books are sequences of at least three cards of various suits with the same numeric value. 
- Runs are sequences of at least three cards of the same suit which have an in order sequences of numeric values.

As seen in the screenshot above, discarding the 10 of Heats can form a book.

Since that would allocate all of the cards in one's hand to groups, the player could then lay down with that hand. 

Seeing this, one can enter 0 as their first selection, signaling that they wish to try to lay down. 

![Discard Prompt for Laying Down](/screenshots/discard-for-laydown.png)

The user would then want to enter the number 3 to represent the third card (10 of Hearts) being selected to discard

Upon successfully laying down, the other player (in this case, the computer) gets to have one more turn to optimize their hand, and then scores up the points in their hand. 

A new round is then started if in a multi-round game with rounds to go & is then displayed to the player, showing the updated scoreboard

![The Start of a New Round](/screenshots/new-round.png)

As seen above, the hand size increases by one at the start of a new round. It is also potentially possible for the other player to be able to lay down their hand on the last turn, which the computer did in this case, leaving both with a score of zero. 

The round sequence goes on and on until the end of game, which is determined by what form of game is being picked. 

There is an end of game readout printed to show the final scores, alongside a message saying who won, or if it was a tie. 

An example end of round score is seen below:

![Final Score](/screenshots/final-score.png)

As seen above, the menu selections come back up, allowing for a new game, different style of game, or for one to exit out of the program

### Scoring: The Meat & Potatoes Of The Game

Mentioned Above, the critical part of the game is how and when one can lay down their hand, as well as what contributes to scoring.

**Key Pieces of Scoring**

- Groups of Cards
  - Runs
  - Books
- Wild Cards
- Round Specific Wild Cards

**Groups of Cards** are collections of at least three cards that fit into a form of scoring criteria. They then negate the points that would be added due to them for both the round and game scores.

**Runs** are Groups that form a sequence of numbers, example 3, 4, 5, that all share the same suit.

**Books** are Groups that form a series of the same numeric value, and can (but are not required to) differ in suit. 

Groups are formed, with the ability to form two groups starting at hand size 6, three groups at hand size 9 and four groups at hand size 12. This in no way means that one needs to form that many groups with that handsize. Groups need to be a minimum of three, and have no maximum of numbers. When scoring points, valid groups are removed from the score, therefore making their cards essentially add zero to the round's total.

**Wild** Cards come in two flavors, there are **Wild** cards, which are of the wild suit, and have a W instead of their numeric value shown. There are also **Round Specific Wild Cards** which are the cards with the numeric value equal to the hand size of the round. Either form of wild card can be used to fill in for any other card in existance when forming groups, making them quite useful to the player. One small downside though is that if they cannot be formed into any group, then they add a ton of points at the end of the game.

If not fit into groups, Wild cards represent the following points

- Hardset Wild Cards represent **50**pts
- Round Specific Wild Cards represent **20**pts

### Winning The Game

The goal of the game is simple, be the person with the least points at the end of the series of rounds.

## Data Types & Decisions:

## Difficulties & Solutions:

## The Good, The Bad, & The Ugly:

## Learning Experience:

First and formost, learning the Language of Rust really brought a lot of new thinking to programming closer to a lower level. Seeing how Rust worked, I found that it brought out more thought to data types than I had thought about in previous experience with C++ oddly, likely due to its different way of handling numeric primitive types. Key Learnings from Rust include:

- Thinking about the differnt way that it declares variables
  - Declaring the type after the name
  - Directly stating if it's mutable, or immutable

# Technologies & Libraries Used

- Rust (programming language)
  - Rand dependency used for shuffle feature

-----

## The following are parts of the ReadMe that will be required for the assignment submission

- Description of your Game
- How to run
- How to play (You need to show screenshots of the gameplay so a person can learn how to play your game)
    - A YouTube video showing your program running and explaining how it goes.
- Data types (How you applied some concepts from the book in your code, you can reference/use the comments you wrote in your code)
- Difficulties and Solutions (What challenges you found and how you overcame them)
- The Good, the Bad and the Ugly (What you loved about this experience, what has bad, and what did you disliked)
- Learning Experience (briefly explain what you learn in this assignment)