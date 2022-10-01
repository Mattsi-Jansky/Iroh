use crate::game::Game;

pub fn generate_pgn(sans: &[String], game: &Game) -> String {
    let mut result = String::new();
    let mut i = 1;
    for pair in sans.chunks(2) {
        result = generate_pgn_chunk(result, i, pair);
        i += 1;
    }

    match game {
        Game::Ongoing {..} | Game::IllegalMove {..} => { result += "*"},
        Game::Win {is_first_player_win: true, ..} => { result += "1-0" },
        Game::Win {is_first_player_win: false, ..} => { result += "0-1" },
        Game::Draw{..} => { result += "1/2-1/2" }
    }

    result
}

fn generate_pgn_chunk(mut result: String, index: u8, pair: &[String]) -> String{
    let turn = format!("{}. {}", index, pair[0]);
    if pair.len() > 1 {
        result = format!("{}{} {} ", result, turn, pair[1])
    } else {
        result = format!("{}{} ", result, turn)
    }
    result
}
