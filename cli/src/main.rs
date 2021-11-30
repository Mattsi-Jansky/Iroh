mod fen_display;
mod terminal;

use console::Term;
use iroh::game::Game;
use crate::fen_display::generate_display_from_fen;

fn main() {
    let term = Term::stdout();

    let game = Game::new();
    let fen = game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]);
    }
}
