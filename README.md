# Iroh

Iroh is an in-progress [chess engine](https://en.wikipedia.org/wiki/Chess_engine) written in Rust. It is affectionately named after a fictional character that wished to play board games every day.

## Dependencies

Rust and Cargo. See the official docs: [Install Rust](https://www.rust-lang.org/tools/install)

## Tests

Run tests with `cargo test`

## TODO

* Shouldn't `from_fen` (both of them) return `Result` / do validation?
* Pawn nuance
    * Promotions
    * En Passant
* Castling