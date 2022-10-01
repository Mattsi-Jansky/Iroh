# Iroh

[![Tests](https://github.com/Mattsi-Jansky/Iroh/actions/workflows/tests.yml/badge.svg)](https://github.com/Mattsi-Jansky/Iroh/actions/workflows/tests.yml)

Iroh is an in-progress [chess engine](https://en.wikipedia.org/wiki/Chess_engine) written in Rust. It is affectionately named after a fictional character that wished to play board games every day.

## Dependencies

Rust and Cargo. See the official docs: [Install Rust](https://www.rust-lang.org/tools/install)

## Tests

Run tests with `cargo test`

## Run

Start the CLI interface in your terminal with `cargo run`.

## TODO

* Shouldn't `from_fen` (both of them) return `Result` / do validation?
* Pawn nuance
    * En Passant
* Draws and draw offers
  * Mutual agreement
  * Seventyfive-move rule
  * Three-fold and fifty-move rules (non-automatic variants of previous two)
  * Dead position
  * https://en.wikipedia.org/wiki/Draw_(chess)#Draws_in_all_games
* Resignations
* Performance
  * Track two (or three?) separate SAN lists rather than filtering the one list in `GameState::determine_status`
  * Is there a better way to do `castling_moves.rs` check for whether king would be in check without cloning GameState?
