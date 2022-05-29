use crate::move_result::MoveResult;
use crate::state::status::Status;

pub fn generate_pgn(sans: &[String], move_result: &MoveResult) -> String {
    let mut result = String::new();
    let mut i = 1;
    for pair in sans.chunks(2) {
        result = generate_pgn_chunk(result, i, pair);
        i += 1;
    }

    match move_result {
        MoveResult::OngoingGame{..} => { result += "*"},
        MoveResult::Win {is_first_player_win: true, ..} => { result += "1-0" },
        MoveResult::Win {is_first_player_win: false, ..} => { result += "0-1" },
        MoveResult::Draw{..} => { result += "1/2-1/2" },
        MoveResult::IllegalMove => { panic!("Illegal move cannot generate a PGN") }
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
