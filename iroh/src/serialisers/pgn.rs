use crate::state::status::Status;

pub fn generate_pgn(sans: &[String], status: Status) -> String {
    let mut result = String::new();
    let mut i = 1;
    for pair in sans.chunks(2) {
        result = generate_pgn_chunk(result, i, pair);
        i += 1;
    }

    match status {
        Status::Ongoing => { result += "*"}
        Status::FirstPlayerWin => { result += "1-0" }
        Status::SecondPlayerWin => { result += "0-1" }
        Status::Draw => { result += "1/2-1/2" }
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
