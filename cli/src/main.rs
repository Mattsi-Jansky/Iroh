mod fen_display;

use std::io::Write;
use console::Term;
use iroh::game::Game;
use crate::fen_display::generate_display_from_fen;

fn main() {
    let mut term = Term::stdout();
    term.clear_screen().unwrap();
    let mut game = Game::new();

    loop {
        render(&term, &game);
        term.write_line("").unwrap();
        term.write_line(&*game.get_pgn()).unwrap();
        term.write_line("").unwrap();
        term.write("Your move: ".as_bytes()).unwrap();

        let input = term.read_line().unwrap();
        let result = game.make_move(&*input);

        if let Ok(new_game_state) = result {
            term.clear_screen().unwrap();
            term.write_line("").unwrap();
            game = new_game_state;
        } else {
            term.clear_screen().unwrap();
            term.write_line(&*format!("Sorry, {} is not a legal move.", input)).unwrap();
        }
    }
}

fn render(term: &Term, game: &Game) {
    let fen = game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]).unwrap();
    }
}