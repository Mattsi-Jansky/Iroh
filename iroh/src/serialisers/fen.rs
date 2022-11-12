use crate::state::coordinates::Coordinate;
use crate::state::tile::{Tile};
use crate::state::GameState;

fn coordinate_from_rank_and_file(rank: u8, file: u8) -> Coordinate {
    Coordinate::from_u8_no_bounds_check(file + rank * 8)
}

pub fn parse_fen(fen: &str, game_state: &mut GameState) {
    let mut rank = 7 as u8;
    let mut file = 0 as u8;
    let mut blocks = fen.split_whitespace();

    for char in blocks.next().expect("Invalid FEN syntax").chars() {
        if char.eq(&'/') {
            rank -= 1;
            file = 0;
            continue;
        }
        if char.is_ascii_digit() {
            file += char as u8 - 0x30;
            continue;
        }

        let mut tile = match char {
            'R' => Tile::FIRST_ROOK,
            'r' => Tile::SECOND_ROOK,
            'N' => Tile::FIRST_KNIGHT,
            'n' => Tile::SECOND_KNIGHT,
            'B' => Tile::FIRST_BISHOP,
            'b' => Tile::SECOND_BISHOP,
            'Q' => Tile::FIRST_QUEEN,
            'q' => Tile::SECOND_QUEEN,
            'K' => Tile::FIRST_KING,
            'k' => Tile::SECOND_KING,
            'P' => Tile::FIRST_PAWN,
            'p' => Tile::SECOND_PAWN,
            _ => Tile::EMPTY
        };
        game_state.board[coordinate_from_rank_and_file(rank, file)] = tile;

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
    let mut blank_tiles_count = 0;

    for i in (0..64).rev() {
        let coordinate= Coordinate::from_u8_no_bounds_check(i);
        let tile = game_state.board[coordinate];
        if tile.is_occupied()  {
            if blank_tiles_count > 0 {
                result.push(char::from_digit(blank_tiles_count, 10).unwrap());
                blank_tiles_count = 0;
            };
            let glyph = generate_fen_piece(tile);
            result.push(glyph);
        } else { blank_tiles_count += 1; }


        if coordinate.is_at_start_of_rank() {
            if blank_tiles_count > 0 {
                result.push(char::from_digit(blank_tiles_count, 10).unwrap());
                blank_tiles_count = 0;
            };
            if i > 0 { result.push('/') }
        };
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

fn generate_fen_piece(tile: Tile) -> char {
    let piece_type = match tile {
        Tile::FIRST_ROOK | Tile::SECOND_ROOK => 'r',
        Tile::FIRST_KNIGHT | Tile::SECOND_KNIGHT => 'n',
        Tile::FIRST_BISHOP | Tile::SECOND_BISHOP => 'b',
        Tile::FIRST_QUEEN | Tile::SECOND_QUEEN => 'q',
        Tile::FIRST_KING | Tile::SECOND_KING => 'k',
        Tile::FIRST_PAWN | Tile::SECOND_PAWN => 'p',
        _ => { panic!("This should never happen - piece is not a valid recognised chesspiece") }
    };
    if tile.is_owned_by_first_player() { piece_type.to_uppercase().next().unwrap() } else {piece_type}
}

#[cfg(test)]
mod tests {
    use crate::state::coordinates::Coordinate;
    use super::*;

    #[test]
    fn parse_fen_from_top_of_board_not_bottom() {
        let fen_that_forces_odd_numbered_rank_piece = "8/8/8/4n3/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_that_forces_odd_numbered_rank_piece, &mut game_state);

        let result = game_state.board[Coordinate::E5];
        assert_eq!(Tile::SECOND_KNIGHT, result)
    }

    #[test]
    fn uppercase_is_first_player() {
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_with_uppercase_king, &mut game_state);

        assert_eq!(true, game_state.board[Coordinate::E7].is_owned_by_first_player());
    }

    #[test]
    fn lowercase_is_second_player() {
        let fen_with_lowercase_king = "4k3/8/8/8/8/8/8/8 w KQkq - 0 1";
        let mut game_state = GameState::new();

        parse_fen(fen_with_lowercase_king, &mut game_state);

        assert_eq!(false, game_state.board[Coordinate::E7].is_owned_by_first_player());
    }

    #[test]
    fn generate_starting_game_fen() {
        let state = GameState::new();

        let result = generate_fen(&state);

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", result);
    }

    #[test]
    fn generate_complex_fen() {
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
        let state = GameState::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        let result = generate_fen(&state);

        assert_eq!("8/8/8/8/8/8/8/8 w - - 0 1", result);
    }

    #[test]
    fn given_metadata_says_so_first_player_starts() {
        let state = GameState::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(true, state.is_first_player_turn);
    }

    #[test]
    fn given_metadata_says_so_second_player_starts() {
        let state = GameState::from_fen("8/8/8/8/8/8/8/8 b - - 0 1");

        assert_eq!(false, state.is_first_player_turn);
    }
}
