# Iroh

Iroh is an in-progress [chess engine](https://en.wikipedia.org/wiki/Chess_engine) written in Rust. It is affectionately named after a fictional character that wished to play board games every day.

## Dependencies

Rust and Cargo. See the official docs: [Install Rust](https://www.rust-lang.org/tools/install)

## Tests

Run tests with `cargo test`

## TODO

* Shouldn't `from_fen` (both of them) return `Result` / do validation?
* Is there a better way to do `castling_moves.rs` check for whether king would be in check without cloning GameState?
* Pawn nuance
    * En Passant
* Castling
  * Need to ensure king can't castle if doing so would put them in check while travelling
* Checkmate
* Draws and draw offers
