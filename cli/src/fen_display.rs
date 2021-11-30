pub fn generate_display_from_fen(fen: &str) -> [String; 10] {
    let mut result = [
        String::from("8| "),
        String::from("7| "),
        String::from("6| "),
        String::from("5| "),
        String::from("4| "),
        String::from("3| "),
        String::from("2| "),
        String::from("1| "),
        String::from("   _______________"),
        String::from("   a b c d e f g h"),
    ];

    let mut parts = fen[..fen.len() - 13].split('/');
    let mut rank_index = 0;
    while let Some(part) = parts.next() {
        let line = &mut result[rank_index];

        let mut file_index = 0;
        for c in part.chars() {
            if !c.is_digit(10) {
                line.push(glyph_for(c));
                if file_index != 7 {
                    line.push(' ');
                }
                file_index += 1;
            } else {
                let digit = c as usize - 0x30;
                for i in 0..digit {
                    if (rank_index % 2) == (file_index % 2) {
                        line.push('◻');
                    } else {
                        line.push('◼');
                    }

                    if file_index != 7 {
                        line.push(' ')
                    }
                    file_index += 1;
                }
            }
        }

        rank_index += 1;
    }

    result
}

fn glyph_for(c: char) -> char {
    match c {
        'R' => '♖',
        'N' => '♘',
        'B' => '♗',
        'Q' => '♕',
        'K' => '♔',
        'P' => '♙',

        'r' => '♜',
        'n' => '♞',
        'b' => '♝',
        'q' => '♛',
        'k' => '♚',
        'p' => '♟',

        _ => panic!("Unknown chess notation, cannot render")
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use super::*;

    #[test]
    fn generate_starting_game_view() {
        let result = generate_display_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(result, [
            String::from("8| ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜"),
            String::from("7| ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟"),
            String::from("6| ◻ ◼ ◻ ◼ ◻ ◼ ◻ ◼"),
            String::from("5| ◼ ◻ ◼ ◻ ◼ ◻ ◼ ◻"),
            String::from("4| ◻ ◼ ◻ ◼ ◻ ◼ ◻ ◼"),
            String::from("3| ◼ ◻ ◼ ◻ ◼ ◻ ◼ ◻"),
            String::from("2| ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙"),
            String::from("1| ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖"),
            String::from("   _______________"),
            String::from("   a b c d e f g h"),
        ]);
    }
}