pub fn generate_display_from_fen(fen: &str) -> [String; 8] {
    let mut result = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
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
            String::from("r n b q k b n r"),
            String::from("p p p p p p p p"),
            String::from("◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼"),
            String::from("P P P P P P P P"),
            String::from("R N B Q K B N R"),
        ]);
    }
}