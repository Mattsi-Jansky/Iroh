mod fen_display;

use std::io::Write;
use console::Term;
use iroh::game::Game;
use crate::fen_display::generate_display_from_fen;

fn main() {
    let mut term = Term::stdout();
    term.clear_screen();
    let mut game = Game::new();

    while true {
        render(&term, &game);
        term.write_line("");
        term.write_line(&*game.get_pgn());
        term.write_line("");
        term.write("Your move: ".as_bytes());

        let input = term.read_line().unwrap();
        let result = game.make_move(&*input);

        if let Ok(new_game_state) = result {
            term.clear_screen();
            term.write_line("");
            game = new_game_state;
        } else {
            term.write_line("");
            term.clear_screen();
            term.write_line(&*format!("Sorry, {} is not a legal move.", input));
        }
    }
}

fn render(term: &Term, game: &Game) {
    let fen = game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]);
    }
}