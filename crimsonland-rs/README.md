# Crimsonland-RS

A Rust port of Crimsonland using the Bevy game engine. This is a top-down twin-stick shooter where you battle endless hordes of creatures while collecting weapons, perks, and power-ups.

## Controls

| Action | Key/Button |
|--------|------------|
| Move | W A S D |
| Aim | Mouse cursor |
| Fire | Left Mouse Button |
| Reload | R |
| Use Item | Space |
| Pause | Escape |

## Game Modes

### Quest Mode
Progress through 53 hand-crafted missions with increasing difficulty. Each quest has specific objectives and creature waves to defeat.

### Survival Mode
Endless mode with progressively harder waves. Survive as long as possible while earning experience and unlocking perks.

### Rush Mode
Time-limited rounds (2 minutes) with pre-selected loadouts. Score-based gameplay with kill streak multipliers:
- 5+ kills: 1.5x multiplier
- 10+ kills: 2.0x multiplier
- 20+ kills: 3.0x multiplier
- 50+ kills: 5.0x multiplier

## Features

- **30+ Weapons**: From pistols and shotguns to plasma rifles and rocket launchers
- **30+ Perks**: Upgrade your character with abilities like regeneration, faster reload, and critical hits
- **15+ Creature Types**: Face zombies, spiders, aliens, ghosts, and massive bosses
- **Power-ups**: Collect bonuses like health, ammo, speed boosts, and weapon pickups
- **Experience & Leveling**: Gain XP from kills to level up and choose new perks

## Building & Running

### Prerequisites
- Rust 1.75+ (install via [rustup](https://rustup.rs/))

### Run (Debug)
```bash
cargo run
```

### Run (Release - Optimized)
```bash
cargo run --release
```

### Run Tests
```bash
cargo test
```

## Project Structure

```
src/
├── main.rs          # App setup and plugins
├── states.rs        # Game state machine
├── player/          # Player entity, movement, shooting
├── creatures/       # Enemy AI, spawning, types
├── weapons/         # Weapon registry and projectiles
├── perks/           # Perk system and effects
├── bonuses/         # Power-ups and collectibles
├── quests/          # Quest database and progression
├── survival.rs      # Survival mode logic
├── rush.rs          # Rush mode logic
├── effects/         # Particles and visual effects
├── ui/              # HUD, menus, perk selection
└── audio/           # Sound effects and music
```

## Tech Stack

- **[Bevy](https://bevyengine.org/)** - ECS game engine
- **[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)** - Audio plugin
- **[serde](https://serde.rs/) + [ron](https://github.com/ron-rs/ron)** - Serialization for configs/saves
- **[rand](https://rust-random.github.io/book/)** - Random number generation

## License

MIT
