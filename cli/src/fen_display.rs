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
                line.push(c);
                if file_index != 7 {
                    line.push(' ');
                }
                file_index += 1;
            } else {
                let digit = c as usize - 0x30;
                for i in 0..digit {
                    line.push('◼');
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
            String::from("8| r n b q k b n r"),
            String::from("7| p p p p p p p p"),
            String::from("6| ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("5| ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("4| ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("3| ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("2| P P P P P P P P"),
            String::from("1| R N B Q K B N R"),
            String::from("   _______________"),
            String::from("   a b c d e f g h"),
        ]);
    }
}