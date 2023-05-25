mod fen_display;

use std::io::Write;

use crate::fen_display::generate_display_from_fen;
use console::Term;
use iroh::game::Game;
use iroh::state::GameState;

fn main() {
    let mut term = Term::stdout();
    term.clear_screen().unwrap();
    let mut game = Game::new();
    let mut input = String::new();

    loop {
        match &game {
            Game::Ongoing { .. } => {
                render(&term, &game, game.generate_fen().unwrap());
                ask_for_next_move(&mut term, &mut input);
                game = game.make_move_san(&*input);
            }
            Game::IllegalMove { state: inner_game } => {
                println!("Sorry, that isn't a legal move. Make sure you write your move using Standard Algebraic Notation.");
                println!("The following moves are available: {:?}", inner_game.get_available_moves().iter().map(|m| m.generate_san()).collect::<Vec<String>>());
                ask_for_next_move(&mut term, &mut input);
                game = game.make_move_san(&*input);
            }
            Game::Draw { state: inner_game }
            | Game::Win {
                state: inner_game, ..
            } => {
                end_game(&mut term, &game, inner_game);
                break;
            }
        }
    }
}

fn ask_for_next_move(term: &mut Term, input: &mut String) {
    term.write_all("Your move: ".as_bytes()).unwrap();
    *input = term.read_line().unwrap();
}

fn end_game(term: &mut Term, game: &Game, game_state: &GameState) {
    term.write_line("").unwrap();
    render(term, game, game_state.generate_fen());
    term.write_line("----------------------").unwrap();
    match game {
        Game::Draw { .. } => term.write_line("It is a draw!").unwrap(),
        Game::Win {
            is_first_player_win: true,
            ..
        } => term
            .write_line("Check mate. Game over! First player wins")
            .unwrap(),
        Game::Win {
            is_first_player_win: false,
            ..
        } => term
            .write_line("Check mate. Game over! Second player wins")
            .unwrap(),
        Game::Ongoing { .. } | Game::IllegalMove { .. } => {
            panic!("Cannot end game, game is not finished")
        }
    };
}

fn render(term: &Term, game: &Game, fen: String) {
    term.clear_screen().unwrap();
    let display = generate_display_from_fen(&fen[..]);

    for line in display {
        term.write_line(&line[..]).unwrap();
    }

    term.write_line("").unwrap();
    term.write_line(game.generate_pgn().unwrap().as_str())
        .unwrap();
    term.write_line(fen.as_str()).unwrap();
    term.write_line("").unwrap();
}
