# Battleship Game in Rust

Welcome to the Battleship game implemented in Rust for the terminal! This game features a pass-and-play multiplayer mode and a single-player mode against a computer opponent. The computer opponent comes with three difficulty levels: easy, medium, and hard. The difficulty levels vary in their attack strategies, from random guessing to more advanced probabilistic calculations.

## How to Play

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/cqb13/terminal-battleship.git
   ```

2. **Navigate to the Project Directory:**

   ```bash
   cd terminal-battleship
   ```

3. **Compile and Run:**

   ```bash
   cargo run
   ```

4. **Follow the On-screen Instructions:**
   - Choose between single-player and pass-and-play multiplayer.
   - Select the difficulty level for the computer opponent (if playing single-player).
   - Enjoy the classic Battleship game experience!

**Note:** Might not work on windows

## Game Features

### Game Board Legend

- `⊕`: Targeted
- `⊗`: Already Attacked
- `🅇`: Hit
- `⓪`: Miss
- `•`: Unknown
- `▧`: Carrier
- `#`: Battleship
- `▭`: Cruiser or Submarine
- `△`: Destroyer

### Player Options

- **Single Player:**
  - Face off against the computer with different difficulty levels.
- **Multiplayer:**
  - Pass and play with a friend.

### Attack Strategies

1. **Easy Difficulty:**

   - Computer makes random guesses.

2. **Medium Difficulty:**

   - Computer uses a hunt-and-target strategy to find and sink ships.

3. **Hard Difficulty:**
   - Computer employs probability-based attacks to target the most likely spaces.

## Code Structure

- `display`: Module for displaying game-related information.
- `game`: Module containing game logic and player/computer interactions.
- `utils`: Utility module for common functionalities.

## Implementation Details

- **Game Board:**

  - The game board is represented as a 2D array of `Tile` enums within the `GameBoard` struct.
  - Each `Tile` enum represents different states of a cell, such as `Targeted`, `AlreadyAttacked`, `Hit`, `Miss`, `Unknown`, or `Ship`.

- **Attack Strategies:**

  - The computer's attack strategy is encapsulated within the `Computer` struct, utilizing various attack strategies based on the difficulty level.
  - The attack strategies include:
    - `RandomAttackStrategy`: Randomly selects attack positions.
    - `HuntAndTargetAttackStrategy`: Seeks out nearby ship positions after a hit.
    - `ProbabilityAttackStrategy`: Uses probability calculations for optimal attacks.

- **Player and Turns:**

  - Players are represented by the `Player` enum, with values `PlayerOne` and `PlayerTwo`.
  - Turn management is implemented in the `multiplayer_game` function, where players take alternating turns.

- **Game Logic:**

  - The core game logic is implemented in the `singleplayer_game` and `multiplayer_game` functions.
  - These functions handle the setup, turns, and win conditions for both single-player and multiplayer modes.

- **Ships and Ship Types:**

  - Ships are represented by the `Ship` struct, containing a `ShipType` enum and orientation information.
  - The `ShipType` enum defines ship names, sizes, and symbols for display.

- **Position and Board Management:**

  - The `Position` struct represents a 2D position on the game board.
  - The `GameBoard` struct manages the overall game board, providing methods for placing markers and checking game state.

- **Constants:**

  - `GRID_SIZE`: A constant indicating the size of the game board.
  - `GRID_ARRAY_SIZE`: A constant representing the array size for the game board.

- **Game Modes and Difficulty Levels:**
  - The `GameMode` enum distinguishes between single-player and multiplayer modes.
  - The `Difficulty` enum defines difficulty levels for the single-player game.

## Contributions

Feel free to contribute to the project by opening issues, suggesting improvements, or adding new features. Pull requests are welcome!

Enjoy playing Battleship in Rust! 🚢🔥
