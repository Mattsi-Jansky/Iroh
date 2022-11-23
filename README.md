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

* Pawn nuance
    * En Passant
* Draws and draw offers
  * Mutual agreement
  * Three-fold and fifty-move rules (non-automatic variants of five and seventyfive-move rules)
  * Dead position
  * https://en.wikipedia.org/wiki/Draw_(chess)#Draws_in_all_games
* Resignations
* Heuristics
  * Penalties for undefended pieces
  * Doubled, blocked, isolated pawns
  * Castling rights (penalty if king can't castle)
  * Centre control
  * King defense
  * Separate weightings of piece material and mobility
  * Checking & checkmate
  * Pieces
    * Knights 
      * Decrease in value as pawns decrease
      * Trapped knight (A8/H8/A7/H7 or A1/H1/A2/H2)
    * Bishops
      * Small bonus for having both (covering both colours)
      * Trapped?
    * Rooks
      * Increasing value as pawns reduce
      * Open file
    * Queen
      * Penalty for moving too soon
* Search
* Performance, readability, etc refactorings
  * Track two (or three?) separate SAN lists rather than filtering the one list in `GameState::determine_status`
  * Is there a better way to do `castling_moves.rs` check for whether king would be in check without cloning GameState?
  * Shouldn't `from_fen` (both of them) return `Result` / do validation?
  * Replace some of these tuples with types, particularly in `board.rs`
  * Searching/evaluating more than needs be in some places
    * `check.rs` should stop further checks if result is true
  * Remove the specific pawn moves, not sure we need them anymore. Could just generate RegularMove/AttackMove
  * `CapturedPieces` struct could do with better encapsulation
* Optimisations
  * When in check only search king moves, and moves that would place a piece between the attacker and king or capture the attacker.  
  * When in double check only search king moves
