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

### Difference Between Card Game & This Game

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