mod fen_display;

use std::io::Write;
use console::Term;
use iroh::game::Game;
use iroh::state::status::Status;
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
            match new_game_state.status() {
                Status::Ongoing => {
                    term.write_line("").unwrap();
                    game = new_game_state;
                },
                Status::FirstPlayerWin | Status::SecondPlayerWin | Status::Draw => {
                    end_game(&mut term, &new_game_state);
                    break;
                }
            }
        } else {
            term.write_line(&*format!("Sorry, {} is not a legal move.", input)).unwrap();
        }
    }
}

fn end_game(term: &mut Term, new_game_state: &Game) {
    term.write_line("").unwrap();
    render(&term, &new_game_state);
    term.write_line("----------------------").unwrap();
    match new_game_state.status() {
        Status::FirstPlayerWin => term.write_line("Check mate. Game over! First player wins"),
        Status::SecondPlayerWin => term.write_line("Check mate. Game over! Second player wins"),
        Status::Draw => term.write_line("It is a draw!"),
        Status::Ongoing => Ok(())
    };
}

fn render(term: &Term, game: &Game) {
    let fen = game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]).unwrap();
    }
}