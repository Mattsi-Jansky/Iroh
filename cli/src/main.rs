mod fen_display;

use std::io::Write;
use console::Term;
use iroh::game::GameInner;
use iroh::move_result::Game;
use crate::fen_display::generate_display_from_fen;

fn main() {
    let mut term = Term::stdout();
    term.clear_screen().unwrap();
    let mut game = GameInner::new();
    let mut input = String::new();

    loop {
        match &game {
            Game::Ongoing {game: inner_game} => {
                render(&term, &game, &inner_game);
                term.write_all("Your move: ".as_bytes()).unwrap();
                input = term.read_line().unwrap();
                game = inner_game.make_move(&*input);
                // if let Ok(new_game_state) = result {
                //     match new_game_state.status() {
                //         Status::Ongoing => {
                //             term.write_line("").unwrap();
                //             game = new_game_state;
                //         },
                //         Status::FirstPlayerWin | Status::SecondPlayerWin | Status::Draw => {
                //             end_game(&mut term, &new_game_state);
                //             break;
                //         }
                //     }
                // }
            }
            Game::IllegalMove => { term.write_line(&*format!("Sorry, {} is not a legal move.", input)).unwrap(); }
            Game::Draw { .. } => {}
            Game::Win { .. } => {}
        }
    }
}

fn end_game(term: &mut Term, game: &Game, inner_game: &GameInner) {
    term.write_line("").unwrap();
    render(term, game, inner_game);
    term.write_line("----------------------").unwrap();
    // match new_game_state.status() {
    //     Status::FirstPlayerWin => term.write_line("Check mate. Game over! First player wins").unwrap(),
    //     Status::SecondPlayerWin => term.write_line("Check mate. Game over! Second player wins").unwrap(),
    //     Status::Draw => term.write_line("It is a draw!").unwrap(),
    //     Status::Ongoing => ()
    // };
}

fn render(term: &Term, game: &Game, inner_game: &GameInner) {
    term.clear_screen().unwrap();
    let fen = inner_game.generate_fen();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]).unwrap();
    }

    term.write_line("").unwrap();
    term.write_line(game.generate_pgn().unwrap().as_str()).unwrap();
    term.write_line(fen.as_str());
    term.write_line("").unwrap();
}
