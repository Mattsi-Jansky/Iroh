use crate::serialisers::fen::parse_fen;
use crate::state::board::Board;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::check::is_check;

pub mod board;
pub mod piece;
pub mod coordinates;
pub mod captured_pieces;
mod check;

const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    turn_number: u16,
    pub is_first_player_turn: bool,
    pub board: Board,
    pub captured_pieces: CapturedPieces,
    pub first_player_can_castle_kingside: bool,
    pub first_player_can_castle_queenside: bool,
    pub second_player_can_castle_kingside: bool,
    pub second_player_can_castle_queenside: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState::from_fen(STARTING_POSITION_FEN)
    }

    pub fn from_fen(fen: &str) -> GameState {
        let mut state = GameState {
            turn_number: 1,
            is_first_player_turn: true,
            board: Board::blank(),
            captured_pieces: CapturedPieces::new(),
            first_player_can_castle_kingside: false,
            first_player_can_castle_queenside: false,
            second_player_can_castle_kingside: false,
            second_player_can_castle_queenside: false,
        };

        parse_fen(fen, &mut state);

        state
    }

    pub fn next_turn(&mut self) {
        self.turn_number += 1;
        self.is_first_player_turn = !self.is_first_player_turn;
    }

    pub(crate) fn is_check(&self, first_player : bool) -> bool {
        is_check(first_player, self)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use crate::state::coordinates::{File, Rank};
    use crate::state::piece::{Piece, PieceType};
    use super::*;

    #[test]
    fn odd_numbered_turns_are_first_player_turns() {
        let game_state = GameState::new();

        assert!(game_state.is_first_player_turn);
    }

    #[test]
    fn even_numbered_turns_are_not_first_player_turns() {
        let mut game_state = GameState::new();

        game_state.next_turn();

        assert_eq!(false, game_state.is_first_player_turn);
    }

    #[test]
    fn create_board_from_fen_layout() {
        let result = GameState::from_fen(STARTING_POSITION_FEN).board;

        assert_that!(&result[(File::new(0),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Rook)));
        assert_that!(&result[(File::new(1),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Knight)));
        assert_that!(&result[(File::new(2),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Bishop)));
        assert_that!(&result[(File::new(3),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Queen)));
        assert_that!(&result[(File::new(4),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::King)));
        assert_that!(&result[(File::new(5),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Bishop)));
        assert_that!(&result[(File::new(6),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Knight)));
        assert_that!(&result[(File::new(7),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Rook)));

        assert_that!(&result[(File::new(0),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(1),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(2),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(3),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(4),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(5),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(6),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(7),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));

        assert!(result[(File::new(0),Rank::new(2))].is_none());
        assert!(result[(File::new(1),Rank::new(2))].is_none());
        assert!(result[(File::new(2),Rank::new(2))].is_none());
        assert!(result[(File::new(3),Rank::new(2))].is_none());
        assert!(result[(File::new(4),Rank::new(2))].is_none());
        assert!(result[(File::new(5),Rank::new(2))].is_none());
        assert!(result[(File::new(6),Rank::new(2))].is_none());
        assert!(result[(File::new(7),Rank::new(2))].is_none());

        assert!(result[(File::new(0),Rank::new(3))].is_none());
        assert!(result[(File::new(1),Rank::new(3))].is_none());
        assert!(result[(File::new(2),Rank::new(3))].is_none());
        assert!(result[(File::new(3),Rank::new(3))].is_none());
        assert!(result[(File::new(4),Rank::new(3))].is_none());
        assert!(result[(File::new(5),Rank::new(3))].is_none());
        assert!(result[(File::new(6),Rank::new(3))].is_none());
        assert!(result[(File::new(7),Rank::new(3))].is_none());

        assert!(result[(File::new(0),Rank::new(4))].is_none());
        assert!(result[(File::new(1),Rank::new(4))].is_none());
        assert!(result[(File::new(2),Rank::new(4))].is_none());
        assert!(result[(File::new(3),Rank::new(4))].is_none());
        assert!(result[(File::new(4),Rank::new(4))].is_none());
        assert!(result[(File::new(5),Rank::new(4))].is_none());
        assert!(result[(File::new(6),Rank::new(4))].is_none());
        assert!(result[(File::new(7),Rank::new(4))].is_none());

        assert!(result[(File::new(0),Rank::new(5))].is_none());
        assert!(result[(File::new(1),Rank::new(5))].is_none());
        assert!(result[(File::new(2),Rank::new(5))].is_none());
        assert!(result[(File::new(3),Rank::new(5))].is_none());
        assert!(result[(File::new(4),Rank::new(5))].is_none());
        assert!(result[(File::new(5),Rank::new(5))].is_none());
        assert!(result[(File::new(6),Rank::new(5))].is_none());
        assert!(result[(File::new(7),Rank::new(5))].is_none());

        assert_that!(&result[(File::new(0),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(1),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(2),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(3),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(4),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(5),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(6),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(7),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));

        assert_that!(&result[(File::new(0),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Rook)));
        assert_that!(&result[(File::new(1),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Knight)));
        assert_that!(&result[(File::new(2),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Bishop)));
        assert_that!(&result[(File::new(3),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Queen)));
        assert_that!(&result[(File::new(4),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::King)));
        assert_that!(&result[(File::new(5),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Bishop)));
        assert_that!(&result[(File::new(6),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Knight)));
        assert_that!(&result[(File::new(7),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Rook)));
    }
}
