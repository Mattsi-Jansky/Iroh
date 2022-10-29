use std::collections::HashMap;
use crate::game::Game;
use crate::moves::Move;
use crate::moves::move_generation::generate_moves;
use crate::moves::resolve_move::resolve_move;
use crate::serialisers::fen::{generate_fen, parse_fen};
use crate::state::board::Board;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::check::is_check;

pub mod board;
pub mod piece;
pub mod coordinates;
pub mod captured_pieces;
mod check;

const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub(crate) sans: Vec<String>,
    pub possible_moves: Vec<Move>,
    pub turn_number: u16,
    pub is_first_player_turn: bool,
    pub board: Board,
    pub captured_pieces: CapturedPieces,
    pub first_player_can_castle_kingside: bool,
    pub first_player_can_castle_queenside: bool,
    pub second_player_can_castle_kingside: bool,
    pub second_player_can_castle_queenside: bool
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
            possible_moves: vec![],
            sans: vec![]
        };
        parse_fen(fen, &mut state);
        state.possible_moves = generate_moves(&state, state.is_first_player_turn);

        state
    }

    pub fn next_turn(&mut self) {
        self.turn_number += 1;
        self.is_first_player_turn = !self.is_first_player_turn;
    }

    pub(crate) fn is_check(&self, is_first_player: bool) -> bool {
        is_check(is_first_player, self)
    }

    pub fn generate_fen(&self) -> String {
        generate_fen(self)
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        self.possible_moves.clone()
    }

    pub fn is_first_player_turn(&self) -> bool {
        self.is_first_player_turn
    }

    pub fn make_move(&self, requested_move: &Move) -> Game {
        if self.possible_moves.contains(requested_move) {
            self.make_move_inner(requested_move)
        } else {
            Game::IllegalMove { state: self.clone() }
        }
    }

    pub fn make_move_san(&self, san: &str) -> Game {
        let mut possible_moves: HashMap<String, &Move> = self.possible_moves.iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.remove(&san.to_string()) {
            self.make_move_inner(requested_move)
        } else {
            Game::IllegalMove { state: self.clone() }
        }
    }

    fn make_move_inner(&self, requested_move: &Move) -> Game {
        let mut sans = self.sans.clone();
        sans.push(String::from(requested_move.generate_san()));
        let game_state = resolve_move(&requested_move, self.clone());
        let possible_moves = generate_moves(&game_state, game_state.is_first_player_turn);
        let state = GameState {
            sans,
            possible_moves,
            ..game_state
        };

        state.determine_status()
    }

    pub(crate) fn determine_status(self) -> Game {
        if self.possible_moves.is_empty() {
            if self.is_check(self.is_first_player_turn) {
                Game::Win{ is_first_player_win: !self.is_first_player_turn(), state: self }
            }
            else { Game::Draw{ state: self} }
        }
        else {
            let mut is_first_player_turn = !self.is_first_player_turn;
            let mut first_player_sans = vec![];
            let mut second_player_sans = vec![];
            for san in self.sans.clone().into_iter().rev() {
                if is_first_player_turn {
                    first_player_sans.push(san);
                } else {
                    second_player_sans.push(san);
                }
                is_first_player_turn = !is_first_player_turn;
            }

            if (!self.is_first_player_turn && self.is_fivefold_repetition(&first_player_sans))
                || self.is_fivefold_repetition(&second_player_sans)
                || self.turn_number - self.captured_pieces.last_capture_turn >= 75{
                Game::Draw{ state: self }
            } else { Game::Ongoing { state: self } }
        }
    }

    fn is_fivefold_repetition(&self, first_player_sans: &Vec<String>) -> bool {
        first_player_sans.len() >= 5
            && first_player_sans[0] == first_player_sans[2]
            && first_player_sans[2] == first_player_sans[4]
            && first_player_sans[1] == first_player_sans[3]
    }

    pub fn captured_pieces(&self) -> &CapturedPieces {
        &self.captured_pieces
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
    use crate::moves::Move::PawnMove;

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

    #[test]
    fn make_move_from_struct_not_san() {
        let state = GameState::new();
        let legal_move = PawnMove((File::E, Rank::TWO), Rank::FOUR);

        let result = state.make_move(&legal_move);

        assert_that!(matches!(result, Game::Ongoing {..}))
    }

    #[test]
    fn given_move_not_in_possible_moves_returns_illegal_move() {
        let state = GameState::new();
        let illegal_move = PawnMove((File::A, Rank::EIGHT), Rank::SIX);

        let result = state.make_move(&illegal_move);

        assert_that!(matches!(result, Game::IllegalMove {..}))
    }
}
