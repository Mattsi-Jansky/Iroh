use crate::moves::Move;

pub fn parse_san(san: &str) -> Move {
    let mut chars = san.chars();
    let column = (chars.next().unwrap() as u32) as usize - 97;
    let row = chars.next().unwrap().to_digit(10).unwrap() as usize - 1;

    Move::PawnMove { 0: column, 1: (column,row) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pawn_move() {
        let result = parse_san("e4");

        assert_eq!(Move::PawnMove(4, (4,3)), result);
    }
}
