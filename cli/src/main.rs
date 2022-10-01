mod fen_display;

use std::io::Write;
use console::Term;
use iroh::game::Game;
use iroh::game_inner::GameInner;
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
                ask_for_next_move(&mut term, &mut input);
                game = inner_game.make_move(&*input);
            }
            Game::IllegalMove { game: inner_game} => {
                term.write_line(&*format!("Sorry, {} is not a legal move.", input)).unwrap();
                ask_for_next_move(&mut term, &mut input);
                game = inner_game.make_move(&*input);
            }
            Game::Draw { game: inner_game } | Game::Win { game: inner_game, .. } => {
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

fn end_game(term: &mut Term, game: &Game, inner_game: &GameInner) {
    term.write_line("").unwrap();
    render(term, game, inner_game);
    term.write_line("----------------------").unwrap();
    match game {
        Game::Draw { .. } => term.write_line("It is a draw!").unwrap(),
        Game::Win { is_first_player_win: true, .. } => term.write_line("Check mate. Game over! First player wins").unwrap(),
        Game::Win { is_first_player_win: false, .. } => term.write_line("Check mate. Game over! Second player wins").unwrap(),
        Game::Ongoing { .. } | Game::IllegalMove { .. } => panic!("Cannot end game, game is not finished")
    };
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
