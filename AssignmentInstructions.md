[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/SHBxUkI4)
# A Game in Rust

The purpose of this assignment are:
1. Learn yet another programming language
2. Practice your creativity
3. Apply the concepts of Chapter 6: Data Types from the book
4. Have an original project that you can add to your portfolio

## Specifications
You will make a program in the Rust programming language. The program will be a game of your choice. The game must not be as simple as Tic-Tac-Toe. Examples of game complexity that are welcome: Battleship, Uno, Texas Hold 'Em Poker, Dungeons and Dragons, Catan, etc. If you are not sure if your game is complex enough, or maybe too complex, feel free to ask your professor to make sure.

Before you start creating your game you will need to study *thoroughly* chapter 6 of the book. The idea is that as you code your game you consciously identify the concepts discussed in that chapter. As you find this concepts in your code you are expected to add comments in the code and make references to the section name from the book.

You will write a **VERY BEAUTIFUL** README file. This file will have the following sections:

* Description of your Game
* How to run
* How to play (You need to show screenshots of the gameplay so a person can learn how to play your game)
  * A YouTube video showing your program running and explaining how it goes.
* Data types (How you applied some concepts from the book in your code, you can reference/use the comments you wrote in your code)
* Difficulties and Solutions (What challenges you found and how you overcame them)
* The Good, the Bad and the Ugly (What you loved about this experience, what has bad, and what did you disliked)
* Learning Experience (briefly explain what you learn in this assignment)

## Rubric
These are the factors to be considered while grading

* Complexity (C), a number [0, 2], when the number is one it means your game is "complex enough," if the value is below 1 means that your program is too simple, and if it is greater than 1 up to 2 it means your program is more complex than expected.
* README (R = [0, 30]), your README file looks good, has all the sections, and it is clear.
* Video (V = [0, 30]), your video is clear, well done, and thorough.
* Data types (DT = [0, 30]), your README and code contain a complete discussion of data types.
* Data types presence (DTP = [0, 1]), you included data type discussion
* Good programming practices (G = -5 per infraction), your code demonstrates good professional programming practices. I will count (n) the number of "bad" programming practices and deduct the points accordingly.
* Subjective assessment (S = [0, 10]), a subjective assessment about the quality of your program.

Your final grade will be calculated as follows:

$Grade=C * DTP * (V+R+DT+S)-n*G$

### Example of Grade Calculation

#### Example 1
* Program: Battleship
* C = 1
* Student didn't include difficulties and learning experiences in the README R = 20
* The video is rushed but otherwise good V = 22
* The README and code included data types discussions DTP = 1
* The data types only discussed arrays DT = 5
* The program showed good programming practices n = 0
* The game looked really good and was fun to play S = 10

$Grade=C * DTP * (V+R+DT+S)-n*G$

$Grade = 1.0 * 1.0 * (22 + 20 + 5 + 10) - 0 * 5 = 57$


#### Example 2
* Program: Dungeons and Dragons
* C = 1.5
* README is complete and well organized README R = 30
* The video is good and thorough V = 30
* The README and code included data types discussions DTP = 1
* The data types discussed several data types in Rust DT = 30
* The program had 4 incidences of bad programming practices (bad indentation, poor choices of variable names, lack of comments, no use of functions) n = 4
* The game looked really good and was fun to play S = 10

$Grade=C * DTP * (V+R+DT+S)-n*G$

$Grade = 1.5 * 1.0 * (30 + 30 + 30 + 10) - 4 * 5 = 130$