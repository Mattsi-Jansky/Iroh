pub fn generate_pgn(sans: &[String]) -> String {
    let number_of_pairs = (sans.len() as f32 / 2.0).ceil() as u8;
    let mut result = String::new();
    let mut i = 1;
    for pair in sans.chunks(2) {
        result = generate_pgn_chunk(number_of_pairs, result, i, pair);
        i += 1;
    }
    result
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
