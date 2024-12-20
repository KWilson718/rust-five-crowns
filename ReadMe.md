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

While there are many different pieces of data that were a part of this overall game, four main concepts come to mind that are worth discussing

### Usage Of Unsigned Numbers

Wherever possible, I made the decision to use unsigned integers, often of a lower size too, since my goal was to have a program that ran with efficiency. While plenty of usize & u32s were used, lots of the core functionality passes around u8s which led to a smaller ram size when running through things like creating and storing an entire deck and the likes. 

The cases where usize was used primarialy aligned with instances where mathmatical operations were in place to interact with the variables. This was a benefit to those calculations, since they needed a usize to be provided rather than a specified size instead. By providing it directly, it saved time from making a copy to unwrap into a usize at runtime (though this happens a few times throughout too). 

The score is an instance where a u32 was used due to needing a larger number, but still wanting to control the size overall. 

### Usage Of Structs & Enums For Type Definitions

When building out a game with the concept of cards, one finds themselves with a structure of formatting that isn't directly applicable to primitive types, nor something that feels comfortable storing inside of a standard array.

This is where the concept of Structs and Enums being used to represent these concepts made the most sense to me to use. 

Enums being used to define types such as suits & values of cards was extremely useful for defining cards, as well as for quickly referencing them. Suits in particular proved to be valuable as an enum, since I could get a string format of the value out of the enum itself, while also setting only a selection of choices as valid ones too.

Once the concepts of Enums came together for defining various attributes of the card, it made sense to create an easy representation of a card, with different attributes being quick to reference. This likely would have been a class in a language such as C++ or Java, but a struct made sense to use here (partially since it was the only option, though this language works REALLY well with structs).

The benefits that using a struct in this instance provides are quite immense, being able to quickly make them, and index them using the .attribute method rather than numerically. Additionally, there is a lot of built in support for iterating through groups of structs and the likes, which made some of the much more complex functions a lot more feasable given the usage of structs!!!

### Usage Of Vectors

For the groupings of cards, such as a hand, deck, or discard pile, the decision was made to go with a Vector, rather than a fixed size array. While one could have written a very barebones variable sized array to use copies of fixed size arrays, I found that the flexibility of a vector was much more worth it in the long run. Being able to adjust the size on the fly, as well as handle the various indexing and switching with ease led to a much faster time writing, while also benefiting anyone that needed to go back and read the code too. 

Particularly in cases such as the discard pile, which starts at zero and only grows, the vector format proved to be extremely useful for creation of this game's logic. 

### How To Score???

A key operation of this game is its ability to calculate the score of various hands, since this gets used for keeping score, determining the ability to lay down, as well as assisting the computer player with what to discard. 

This also was the most complex portion to program, however more detail on this will be provided in the following sections...

I found that using hash maps, as well as rust's built in ordering with vectors seemed to be the solution to building out the working parts of this concept. This was heavily influenced by seeing a struct's different derived operations, and how they could then be used to add compatibility with hashsets & hashmaps. Both were used for the calculate_score function, which used find_runs & find_books as well. The ability to quickly group and sort effectively proved critical to the functionality of score-tracking.

## Difficulties & Solutions:

Building out a fully functioning card game in command line format, while also having one crazy finals season added up to create many difficulties along the way, however many great solutions were brought about from it as well!

| Problem Encountered | Solution Found & Implemented |
| ----- | -----|
| Time of Development - given the Finals Season, as well as a very busy quarter, with different challenges academically, as well as persoally.  | Having planned ways to shrink the scope if necessary. I had a few key dates that would have led to rescoping if I weren't at a certain point by the time that I reached them. Thankfully this wasn't really an issue, but led to a lot of pushing a specific time to make something happen. |
| New Language: Rust... | Spent time watching some of the basics, and thought through development progress in way to specifically learn on the fly. By starting out simple with things like outputting messages for the start of the game, learning how the variables worked, and such, I felt like I was far more capable when it came to complex ideas like the the scoring |
| No Objects | Was able to get so much use out of structs, and found myself running through different ways of using them to achieve very similar things as I would have with a class. Basically the deck & the card struct would have likely been very similar, just different classes instead of a file filled with functions, and an array full of structs. | 
| Scoring Hands...  **The Big One** | This was the main piece of the program that I genuinely had no clue at how to achieve going in. I kept telling myself that I'd find a way, even when I couldn't have thought of one, and honestly worked through writing **MANY** functions that ended up getting deleted since they weren't used by the final functionality. This additionally was a time of further research into how the Rust Langauge worked, hoping to find something that gave a clue towards the answer. By what I can only describe as divine intervention, I found myself working with Hash concepts, through HashSets & HashMaps. I was then able to run through concepts to make both types of card groupings, and the pieces kept falling together in the most convienent of ways.  |


## The Good, The Bad, & The Ugly:

### The Good

Rust honsestly has been a very fun language to learn through experience. It is well documented, and the error detection that the compiler does is truly amazing. The concepts of ownership and mutability have been immensely fun to learn, leaving me thinking far more about what a variable needs to do in its life than I have with other languages. 

Using the structs, enums, and vectors to make my game worked really well to lay out data in the necessary formats. Each data type really blended well with eachother in terms of usability, with the overall writability of Rust really demonstrating this. 

### The Bad

There are a few pieces of work that could be implemented to lead to a more complete game overall, though they are edge cases that were left out due to time.

Could Do List:

1. Implement the discard pile being shuffled back into the deck when the deck runs out
   1. This would have been top on the priority list if I had a day longer timewise to work on this (finals didn't let me do that)
2. Add in more game variation options
   1. It would have been fun to have the option to specify what number of rounds the player would want for a custom game
3. Add in the option for a second, third, or fourth player

Each of these would have been fun to have gotten to implementing, and might be further ideas to touch on in the future.

Regarding data structures, I used quite a lot of variation between usize, u32, and u8 variables, where it might have honestly been faster to just go with a blanket usage of usize for compatibility. This was a time of experimentation, which decreased consistency across the board.

### The Ugly

#### Scoring

Overall, the worst offender of this list is the calculate_score function, as well as its children, the find_runs & find_books functions. The state that they are in results in no crashing, and a workable feature, however there's far more that could be improved on them. 

Concepts such as optimizing where wild cards were used could have been improved, as well as a few corner cases where a lower score is possible depending on what's grouped with what. Additionally, further logic about prioritizing something's entry into a run or a book could have undergone further development. This could have led to a much smoother running game, but would have needed far more time to develop.

#### DevOps Considering Timing

The development of this game came at a time in my life where I was really low on the amount of time and brainpower that I could give to this assignment. Finals for me was probably my harshest time yet, but the entire quarter was stretching me as thin as possible. Despite starting quite early on the project, I'm typing this section out within 6hrs of the deadline, which is much closer than I'd like to have been. Despite personal life challenges, and needing to academically balance between classes, I somehow finished this, which feels like something that I wouldn't have been able to imagine saying within the last few days. 

## Learning Experience:

First and formost, learning the Language of Rust really brought a lot of new thinking to programming closer to a lower level. Seeing how Rust worked, I found that it brought out more thought to data types than I had thought about in previous experience with C++ oddly, likely due to its different way of handling numeric primitive types. Key Learnings from Rust include:

- Thinking about the differnt way that it declares variables
  - Declaring the type after the name
  - Directly stating if it's mutable, or immutable
  - Understanding ownership throughout the entire program
  - Accounting for far more relating to the concepts in programming course material due to the structure of these variables
  - Not being able to rely on objects to create the abstractions that I wanted quite as much

Additionally, being able to begin to think through how to manage more and more complex algorithms seems like a great way to tee up the next quarter's class specifically on Algorithm Analysis...

---

# Technologies & Libraries Used

- Rust (programming language)
  - Rand dependency used for shuffle feature