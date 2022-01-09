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

    let fen = fen.split(' ').next().unwrap();
    let parts = fen.split('/');
    for (rank_index, part) in parts.enumerate() {
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
                for _i in 0..digit {
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

    #[test]
    fn generate_game_view_with_different_meta_info_length() {
        let result = generate_display_from_fen("rnbqk2r/pppp1ppp/3b1n2/4p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1");

        assert_eq!(result, [
        String::from("8| ♜ ♞ ♝ ♛ ♚ ◼ ◻ ♜"),
        String::from("7| ♟ ♟ ♟ ♟ ◼ ♟ ♟ ♟"),
        String::from("6| ◻ ◼ ◻ ♝ ◻ ♞ ◻ ◼"),
        String::from("5| ◼ ◻ ◼ ◻ ♟ ◻ ◼ ◻"),
        String::from("4| ◻ ◼ ◻ ◼ ♙ ◼ ◻ ◼"),
        String::from("3| ◼ ◻ ◼ ♗ ◼ ♘ ◼ ◻"),
        String::from("2| ♙ ♙ ♙ ♙ ◻ ♙ ♙ ♙"),
        String::from("1| ♖ ♘ ♗ ♕ ◼ ♖ ♔ ◻"),
        String::from("   _______________"),
        String::from("   a b c d e f g h"),
        ]);
    }
}
