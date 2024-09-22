# Slow Life RPG Game

A fun, relaxing RPG game developed in my free time, built with Rust and the Bevy engine. The game features exploration, NPC interactions, and character progression, all in a peaceful, slow-life setting.

## Features
- 16-bit bird's-eye view RPG style
- Multiple maps with diverse environments
- Interaction with NPCs
- Monster encounters with combat mechanics
- Level-up system with stat changes
- Character and environment customization options
- Procedurally generated content (e.g., monsters, levels)

## Future Features

### AI-Powered Monsters
In future updates, I plan to introduce more challenging encounters by developing algorithms for monster AI. The goal is to make each monster more intelligent and adaptive to the player’s behavior. AI features might include:
- Dynamic difficulty scaling based on player level
- Pattern recognition and decision-making (e.g., monsters choosing different attack strategies)
- Flocking and group tactics for monsters to cooperate in combat

### LLM-Powered NPC Interactions
To make the game world more immersive and engaging, I am exploring integrating Large Language Models (LLMs) for NPC dialogue. This would allow players to have dynamic, meaningful conversations with NPCs, enabling:
- Context-aware conversations based on player progress
- Complex dialogue trees that respond naturally to player inputs
- Quest generation based on the player's previous actions and world state
- Personalized responses to make each player's experience unique

## Project Structure
```
slow-life-game/
├── assets/              # Game assets (audio, fonts, images)
│   ├── audio/
│   ├── fonts/
│   ├── images/
├── src/                 # Source code for the game
│   ├── components/      # Game components (e.g., player, NPCs, monsters)
│   ├── resources/       # Resources for the game (e.g., textures, fonts)
│   └── systems/         # Game systems (e.g., movement, combat, interaction)
│   └── utils/           # Utility modules
├── Cargo.toml           # Cargo configuration file
└── README.md            # Project documentation
```

## Dependencies
This project uses the following Rust crates:
- **[Bevy](https://bevyengine.org/)**: The game engine used to build the game.
- **[Rand](https://crates.io/crates/rand)**: Used for generating random content (e.g., monsters, map elements).

To see the complete list of dependencies, check the `Cargo.toml` file.

## Getting Started

### Prerequisites
- Install Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Set up a Bevy project by following the [Bevy getting started guide](https://bevyengine.org/learn/book/getting-started/).

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/BigDataRPG/slow-life-game.git
    cd slow-life-game
    ```

2. Build the game:

    ```bash
    cargo build
    ```

3. Run the game:

    ```bash
    cargo run
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- Thanks to the Bevy community for their support and guidance.
- Special thanks to the creators of the assets used in the game.