# yatzy

[![Crates.io](https://img.shields.io/crates/v/yatzy)](https://crates.io/crates/yatzy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/smearor/dice-rs/actions/workflows/build.yml/badge.svg)](https://github.com/smearor/dice-rs/actions/workflows/build.yml)
[![Book](https://img.shields.io/badge/mdBook-Book-blue)](https://smearor.github.io/dice-rs/book/)

A Kniffel (Yatzy) dice game for physical Bluetooth dice, built with GTK 4 and
the [dice-rs](https://crates.io/crates/dice-rs) library.

## Requirements

**You need [GoDice](https://particula-tech.com/products/godice-full-pack)
physical dice to play this game.** GoDice are Bluetooth Low Energy (BLE) dice
with embedded LEDs and an accelerometer. The game scans for GoDice devices,
connects up to 5 dice, and uses real roll events for gameplay.

The game is powered by [dice-rs](https://github.com/smearor/dice-rs), which
provides the BLE transport layer, dice event streaming, LED control, and
battery monitoring.

## Features

- **Full Kniffel scorecard** with all 13 categories in upper and lower sections
- **Single-player mode** with a computer AI opponent using expected-value
  calculations for optimal decisions
- **Multi-player pass-and-play** mode with turn transitions and hints
  disabled for fairness
- **Strategy hints** showing the best category and best hold decision
  (single-player mode only)
- **Cross-out advisor** recommending the least-damaging category when no
  valid score is possible
- **LED celebration effects** on the physical dice for special rolls:
  Yatzy, Full House, Large Straight, Small Straight, Four-of-a-Kind
- **LED color assignment** per player for visual identification
- **Dice reconnection** manager for handling dropped connections during gameplay
- **Persistent highscore list** (top 20 entries, stored as JSON)
- **Persistent game settings** (player names, colors, types, game mode)
- **Internationalization** with Fluent — German and English translations

## Screenshots

_Coming soon._

## Building

The game requires GTK 4 development libraries:

```sh
# Ubuntu/Debian
sudo apt install libgtk-4-dev

# Build
cargo build -p yatzy
```

## Running

```sh
cargo run -p yatzy
```

The application starts with a setup screen for configuring players (names,
colors, human/computer type) and selecting the game mode. After setup, scan
for GoDice devices and connect. Each player rolls the physical dice, holds
dice between rolls, and enters scores in the scorecard.

## How to Play

1. **Setup**: Choose single-player or multi-player mode, configure player
   names and colors
2. **Connect**: Scan for GoDice devices and connect 5 dice
3. **Roll**: Each player has up to 3 rolls per turn. Roll the physical dice,
   then click dice to hold them between rolls
4. **Score**: After rolling (or after the 3rd roll), select a category to
   enter the score. If no category fits, cross out the least valuable one
5. **Repeat**: 13 rounds total, then the player with the highest score wins

### Scorecard Categories

| Section | Category | Rule | Score |
|---------|----------|------|-------|
| Upper | Ones–Sixes | Sum of matching values | Count × value |
| Upper | Bonus | Upper section ≥ 63 | +35 |
| Lower | Three-of-a-Kind | ≥ 3 same value | Sum of all dice |
| Lower | Four-of-a-Kind | ≥ 4 same value | Sum of all dice |
| Lower | Full House | 3 + 2 of same values | 25 |
| Lower | Small Straight | 4 consecutive values | 30 |
| Lower | Large Straight | 5 consecutive values | 40 |
| Lower | Yatzy | All 5 dice same value | 50 |
| Lower | Chance | Any combination | Sum of all dice |

## Platform Support

- **Linux** only — requires GTK 4 and a Bluetooth adapter (BlueZ 5.x)

## Related Crates

- [dice-rs](https://crates.io/crates/dice-rs) — Core BLE library for GoDice
- [dice-rs-cli](https://crates.io/crates/dice-rs-cli) — CLI tool for GoDice
- [dice-rs-controller](https://crates.io/crates/dice-rs-controller) — GTK 4
  desktop app with 3D dice rendering
- [dice-rs-ws](https://crates.io/crates/dice-rs-ws) — WebSocket server for
  GoDice events

## Documentation

- **User Guide**: [mdBook](https://smearor.github.io/dice-rs/book/) (includes
  a [Yatzy chapter](https://smearor.github.io/dice-rs/book/yatzy.html))
- **API Reference**: [docs.rs](https://docs.rs/yatzy)
- **Changelog**: [CHANGELOG.md](../../CHANGELOG.md)

## License

Licensed under the [MIT License](../../LICENSE).
