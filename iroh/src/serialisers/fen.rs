use crate::state::coordinates::{File, Rank};
use crate::state::piece::{Piece, PieceType};
use crate::state::GameState;

pub fn parse_fen(fen: &str, callback: &mut dyn FnMut((File, Rank), Option<Piece>)) {
    let mut rank = Rank::new(7);
    let mut file = File::new(0);

    for char in fen[..fen.len() - 13].chars() {
        if char.eq(&'/') {
            rank -= 1;
            file = File::new(0);
            continue;
        }
        if char.is_digit(10) {
            file += char as usize - 0x30;
            continue;
        }

        let piece_type = match char {
            'r' | 'R' => Some(PieceType::Rook),
            'n' | 'N' => Some(PieceType::Knight),
            'b' | 'B' => Some(PieceType::Bishop),
            'q' | 'Q' => Some(PieceType::Queen),
            'k' | 'K' => Some(PieceType::King),
            'p' | 'P' => Some(PieceType::Pawn),
            _ => None
        };
        let is_owned_by_first_player = char.is_uppercase();
        callback((file, rank),
                 piece_type.map(
                     |piece_type| Piece::new(is_owned_by_first_player, piece_type)
                 ));

        file += 1;
    }
}

pub fn generate_fen(game_state: &GameState) -> String {
    let mut result = String::new();

    for r in (0..=Rank::MAX).rev() {
        let mut empty_aggr = 0;
        for f in 0..=File::MAX {
            if let Some(piece) = game_state.board[(File::new(f),Rank::new(r))] {
                if empty_aggr > 0 { result.push(char::from_digit(empty_aggr, 10).unwrap()) };
                let glyph = generate_fen_piece(piece);
                result.push(glyph);
            } else { empty_aggr += 1; }
        }

        if empty_aggr > 0 { result.push(char::from_digit(empty_aggr, 10).unwrap()) };
        if r > 0 { result.push('/') };
    }

    result.push_str(" w KQkq - 0 1");
    result
}

fn generate_fen_piece(piece: Piece) -> char {
    let piece_type = match piece.piece_type {
        PieceType::Rook => 'r',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::Queen => 'q',
        PieceType::King => 'k',
        PieceType::Pawn => 'p',
    };
    if piece.is_owned_by_first_player { piece_type.to_uppercase().next().unwrap() } else {piece_type}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fen_from_top_of_board_not_bottom() {
        let mut result: Vec<(File,Rank)> = vec![];
        let fen_that_forces_odd_numbered_rank_piece = "8/8/8/4n3/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_that_forces_odd_numbered_rank_piece, &mut |(file, rank), _piece| {
            result.push((file, rank));
        });

        assert_eq!(1, result.len());
        let result = result.get(0).unwrap();
        assert_eq!(4, result.0);
        assert_eq!(4, result.1);
    }

    #[test]
    fn uppercase_is_first_player() {
        let mut result: Vec<Piece> = vec![];
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_with_uppercase_king, &mut |(_,_),piece| {
            result.push(piece.unwrap());
        });

        assert_eq!(1, result.len());
        assert_eq!(true, result.get(0).unwrap().is_owned_by_first_player);
    }

    #[test]
    fn lowercase_is_second_player() {
        let mut result: Vec<Piece> = vec![];
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_with_uppercase_king, &mut |(_,_),piece| {
            result.push(piece.unwrap());
        });

        assert_eq!(1, result.len());
        assert_eq!(true, result.get(0).unwrap().is_owned_by_first_player);
    }

    #[test]
    fn generate_fen_from_game_state() {
        let state = GameState::new();

        let result = generate_fen(&state);

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", result);
    }
}