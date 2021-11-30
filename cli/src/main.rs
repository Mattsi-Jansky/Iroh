mod fen_display;
mod terminal;

use std::io::Write;
use console::Term;
use iroh::game::Game;
use crate::fen_display::generate_display_from_fen;

fn main() {
    let mut term = Term::stdout();
    term.clear_screen();

    let mut game = Game::new();
    render(&term, &game);
    term.write_line("");
    term.write("Your move: ".as_bytes());

    let read = term.read_line();
    term.clear_last_lines(8);
}

fn render(term: &Term, game: &Game) {
    let fen = game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]);
    }
}