use crate::state::coordinates::{File, Rank};
use crate::state::piece::{Piece, PieceType};
use crate::state::GameState;

pub fn parse_fen(fen: &str, game_state: &mut GameState) {
    let mut rank = Rank::new(7);
    let mut file = File::new(0);
    let mut blocks = fen.split_whitespace();

    for char in blocks.next().expect("Invalid FEN syntax").chars() {
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
        game_state.board[(file, rank)] = piece_type.map(
                     |piece_type| Piece::new(is_owned_by_first_player, piece_type));

        file += 1;
    }

    let player_to_move = blocks.next().expect("Invalid FEN syntax");
    match player_to_move {
        "w" => { game_state.is_first_player_turn = true; },
        "b" => { game_state.is_first_player_turn = false; },
        _ => panic!("Invalid FEN syntax")
    }

    for char in blocks.next().expect("Invalid FEN syntax").chars() {
        match char {
            'K' => { game_state.first_player_can_castle_kingside = true; }
            'Q' => { game_state.first_player_can_castle_queenside = true; }
            'k' => { game_state.second_player_can_castle_kingside = true; }
            'q' => { game_state.second_player_can_castle_queenside = true; }
            '-' => { /* Do nothing */ }
            _ => panic!("Invalid FEN syntax")
        };
    }
}

pub fn generate_fen(game_state: &GameState) -> String {
    let mut result = String::new();

    for r in (0..=Rank::MAX).rev() {
        let mut blank_tiles_count = 0;
        for f in 0..=File::MAX {
            if let Some(piece) = game_state.board[(File::new(f),Rank::new(r))] {
                if blank_tiles_count > 0 {
                    result.push(char::from_digit(blank_tiles_count, 10).unwrap());
                    blank_tiles_count = 0;
                };
                let glyph = generate_fen_piece(piece);
                result.push(glyph);
            } else { blank_tiles_count += 1; }
        }

        if blank_tiles_count > 0 { result.push(char::from_digit(blank_tiles_count, 10).unwrap()) };
        if r > 0 { result.push('/') };
    }

    result.push_str(&*format!(
        " {} {} - 0 1",
        if game_state.is_first_player_turn {"w"} else {"b"},
        generate_castling_metadata(game_state)));
    result
}

fn generate_castling_metadata(game_state: &GameState) -> String {
    let mut result = String::new();

    if game_state.first_player_can_castle_kingside { result.push('K')}
    if game_state.first_player_can_castle_queenside { result.push('Q')}
    if game_state.second_player_can_castle_kingside { result.push('k')}
    if game_state.second_player_can_castle_queenside { result.push('q')}

    if result.is_empty() { result = String::from("-") }

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
        let fen_that_forces_odd_numbered_rank_piece = "8/8/8/4n3/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_that_forces_odd_numbered_rank_piece, &mut game_state);

        let result = game_state.board[(File::new(4),Rank::new(4))].unwrap();
        assert_eq!(PieceType::Knight, result.piece_type)
    }

    #[test]
    fn uppercase_is_first_player() {
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_with_uppercase_king, &mut game_state);

        assert_eq!(true, game_state.board[(File::new(4), Rank::new(7))].unwrap().is_owned_by_first_player);
    }

    #[test]
    fn lowercase_is_second_player() {
        let fen_with_lowercase_king = "4k3/8/8/8/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_with_lowercase_king, &mut game_state);

        assert_eq!(false, game_state.board[(File::new(4), Rank::new(7))].unwrap().is_owned_by_first_player);
    }

    #[test]
    fn generate_starting_game_fen() {
        let state = GameState::new();

        let result = generate_fen(&state);

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", result);
    }

    #[test]
    fn generate_complex_fen() {
        //Technically this FEN is illegal because the kings shouldn't be able to castle,
        //but we haven't implemented castling at time of writing
        let test_fen = "rnBNkQn1/ppppNRpp/1PP2p2/2B1p3/2q2Kb1/5r2/2PPPPPP/R7 w KQkq - 0 1";
        let state = GameState::from_fen(test_fen);

        let result = generate_fen(&state);

        assert_eq!(test_fen, result);
    }

    #[test]
    fn generate_metadata_can_first_player_castle_kingside() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        state.first_player_can_castle_kingside = false;

        let result = generate_fen(&state);

        assert_eq!("8/8/8/8/8/8/8/8 w Qkq - 0 1", result);
    }

    #[test]
    fn generate_metadata_can_first_player_castle_queenside() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        state.first_player_can_castle_queenside = false;

        let result = generate_fen(&state);

        assert_eq!("8/8/8/8/8/8/8/8 w Kkq - 0 1", result);
    }

    #[test]
    fn generate_metadata_which_players_turn_is_it() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        state.next_turn();

        let result = generate_fen(&state);

        assert_eq!("8/8/8/8/8/8/8/8 b KQkq - 0 1", result);
    }

    #[test]
    fn given_no_player_can_castle_show_hyphen() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        let result = generate_fen(&state);

        assert_eq!("8/8/8/8/8/8/8/8 w - - 0 1", result);
    }

    #[test]
    fn given_metadata_says_so_first_player_starts() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(true, state.is_first_player_turn);
    }

    #[test]
    fn given_metadata_says_so_second_player_starts() {
        let mut state = GameState::from_fen("8/8/8/8/8/8/8/8 b - - 0 1");

        assert_eq!(false, state.is_first_player_turn);
    }
}
