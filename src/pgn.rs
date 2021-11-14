pub fn generate_pgn(sans: &Vec<String>) -> String {
    let number_of_pairs = (sans.len() as f32 / 2 as f32).ceil() as u8;
    let mut result = String::new();
    let mut i = 1;
    for pair in sans.chunks(2) {
        result = generate_pgn_chunk(number_of_pairs, result, i, pair);
        i = i + 1;
    }
    return result
}

fn generate_pgn_chunk(number_of_pairs: u8, mut result: String, index: u8, pair: &[String]) -> String{
    let turn = format!("{}. {}", index, pair[0]);
    if pair.len() > 1 {
        let asterisk_if_end = if index != number_of_pairs {
            ""
        } else {
            "*"
        };
        result = format!("{}{} {} {}", result, turn, pair[1], asterisk_if_end)
    } else {
        result = format!("{}{} *", result, turn)
    }
    result
}
