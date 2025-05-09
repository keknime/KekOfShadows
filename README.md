# Kek of Shadows

**Kek of Shadows** is a 2.5D isometric MMORPG built with Rust/WebAssembly, Express, and Solana blockchain integration. Players can explore tile-based maps, engage in PvE/PvP combat, customize characters with a skill tree, and manage NFT-based characters on the Solana network. The game includes a map editor, item/monster database, and multiplayer features via WebSocket.

## Features
- **Isometric 2.5D Gameplay**: Explore a fantasy world with tile-based maps (grass, stone, water).
- **Multiplayer**: Real-time player interactions using WebSocket for PvE and PvP.
- **Skill Tree**: Unlock and customize character abilities.
- **Solana NFT Integration**: Characters are stored as NFTs on the Solana blockchain.
- **Map Editor**: Create and edit maps with a web-based tool.
- **Item & Monster Database**: JSON-driven system for equipment and enemies.
- **Professions & Races**: Choose from Warrior, Mage, Archer, or Blacksmith, and Human, Elf, Dwarf, or Orc.

## Tech Stack
- **Frontend**: Pug, JavaScript, WebGL (via Rust/WebAssembly)
- **Backend**: Express.js, WebSocket
- **Game Logic**: Rust compiled to WebAssembly
- **Blockchain**: Solana (Anchor framework)
- **Data**: JSON files for items, monsters, maps, and skill tree

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (for game logic)
- [Node.js](https://nodejs.org/) (for Express server)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (for blockchain integration)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for WebAssembly)
- [Phantom Wallet](https://phantom.app/) (for Solana interactions)
- A modern web browser

### Installation
1. **Clone the Repository**
   ```bash
   git clone https://github.com/your-username/kek-of-shadows.git
   cd kek-of-shadows
   ```

2. **Install Rust Dependencies**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   wasm-pack build --target web --out-dir public --out-name wasm_game
   ```

3. **Install Node.js Dependencies**
   ```bash
   npm install
   ```

4. **Set Up Solana**
   - Configure Solana CLI for the devnet:
     ```bash
     solana config set --url https://api.devnet.solana.com
     ```
   - Deploy the Solana program:
     ```bash
     cd programs/kek_of_shadows
     anchor build
     anchor deploy
     ```

5. **Run the Server**
   ```bash
   node server.js
   ```

6. **Run the WebSocket Server**
   ```bash
   node websocket.js
   ```

7. **Access the Game**
   - Open `http://localhost:3000` in a browser.
   - Connect a Phantom Wallet, create a character, and start exploring!

### Map Editor
- Access the map editor at `http://localhost:3000/map_editor`.
- Design maps with terrain tiles (grass, stone, water) and save them as JSON files in `public/maps`.

## Project Structure
kek-of-shadows/

├── public/                 # Static files (WASM, JS, JSON data)

│   ├── maps/              # Map JSON files

│   ├── game_data.json     # Items and monsters

│   ├── skill_tree.json    # Skill tree configuration

│   ├── wasm_game.js       # WebAssembly bindings

│   └── map_editor.html    # Map editor UI

├── src/                   # Rust game logic

│   └── lib.rs             # Core game logic

├── programs/              # Solana program

│   └── kek_of_shadows/

│       └── src/

│           └── lib.rs     # Solana Anchor program

├── views/                 # Pug templates

│   └── layout.pug         # Main game UI

├── server.js              # Express server

├── websocket.js           # WebSocket server

├── client.js              # Client-side JavaScript

├── .gitignore             # Git ignore rules

├── LICENSE                # MIT License

└── README.md              # This file

## Contributing
Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m 'Add your feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License
This project is licensed under the [MIT License](LICENSE).

## Roadmap
- Add inventory system for item management.
- Implement quests with NPC interactions.
- Overhaul combat with health, mana, and abilities.
- Introduce crafting and trading systems.
- Enhance visuals with sprite sheets and audio.

## Contact
For questions or feedback, open an issue on GitHub or reach out to [your-username].

---
Built with 💻 and ⚔️ for the Solana and Rust communities!
