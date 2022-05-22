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
        term.write_line(&*game.generate_fen()).unwrap();
        term.write_line("").unwrap();
        term.write_all("Your move: ".as_bytes()).unwrap();

        let input = term.read_line().unwrap();
        let result = game.make_move(&*input);

        term.clear_screen().unwrap();
        if let Ok(new_game_state) = result {
            if new_game_state.is_game_ongoing() {
                term.write_line("").unwrap();
                game = new_game_state;
            }
            else {
                term.write_line("").unwrap();
                render(&term, &new_game_state);
                term.write_line("----------------------").unwrap();
                let winning_player_name = if new_game_state.is_first_player_turn() {"Second player"}
                    else {"First player"};
                term.write_line(&format!("Check mate. Game over! {} loses", winning_player_name));
                break;
            }
        } else {
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