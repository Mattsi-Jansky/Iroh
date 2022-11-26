use crate::game::Game;
use crate::moves::move_generation::generate_moves;
use crate::moves::resolve_move::resolve_move;
use crate::moves::Move;
use crate::serialisers::fen::{generate_fen, parse_fen};
use crate::state::board::Board;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::check::is_check;
use std::collections::HashMap;

pub mod board;
pub mod captured_pieces;
pub(crate) mod check;
pub mod coordinates;
pub mod tile;

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
            possible_moves: vec![],
            sans: vec![],
        };
        parse_fen(fen, &mut state);
        let is_first_player_turn = state.is_first_player_turn;
        state.possible_moves = generate_moves(&mut state, is_first_player_turn);

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

    pub fn make_move(&self, requested_move: &Move) -> Option<Self> {
        if self.possible_moves.contains(requested_move) {
            Some(self.make_move_inner(requested_move))
        } else {
            None
        }
    }

    pub fn make_move_san(&self, san: &str) -> Option<Self> {
        let mut possible_moves: HashMap<String, &Move> = self
            .possible_moves
            .iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.remove(&san.to_string()) {
            Some(self.make_move_inner(requested_move))
        } else {
            None
        }
    }

    fn make_move_inner(&self, requested_move: &Move) -> Self {
        let mut sans = self.sans.clone();
        sans.push(requested_move.generate_san());
        let mut game_state = self.clone();
        resolve_move(requested_move, &mut game_state);
        let is_first_player_turn = game_state.is_first_player_turn;
        let possible_moves = generate_moves(&mut game_state, is_first_player_turn);
        GameState {
            sans,
            possible_moves,
            ..game_state
        }
    }

    pub(crate) fn determine_status(self) -> Game {
        if self.possible_moves.is_empty() {
            if self.is_check(self.is_first_player_turn) {
                Game::Win {
                    is_first_player_win: !self.is_first_player_turn(),
                    state: self,
                }
            } else {
                Game::Draw { state: self }
            }
        } else {
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
                || self.turn_number - self.captured_pieces.last_capture_turn >= 75
            {
                Game::Draw { state: self }
            } else {
                Game::Ongoing { state: self }
            }
        }
    }

    pub(crate) fn is_fivefold_repetition(&self, first_player_sans: &Vec<String>) -> bool {
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
    use crate::moves::Move::PawnMove;
    use crate::state::coordinates::Coordinate;
    use galvanic_assert::assert_that;

    use super::*;
    use crate::state::tile::Tile;

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

        assert_eq!(result[Coordinate::A1], Tile::FIRST_ROOK);
        assert_eq!(result[Coordinate::B1], Tile::FIRST_KNIGHT);
        assert_eq!(result[Coordinate::C1], Tile::FIRST_BISHOP);
        assert_eq!(result[Coordinate::D1], Tile::FIRST_QUEEN);
        assert_eq!(result[Coordinate::E1], Tile::FIRST_KING);
        assert_eq!(result[Coordinate::F1], Tile::FIRST_BISHOP);
        assert_eq!(result[Coordinate::G1], Tile::FIRST_KNIGHT);
        assert_eq!(result[Coordinate::H1], Tile::FIRST_ROOK);

        assert_eq!(result[Coordinate::A2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::B2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::C2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::D2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::E2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::F2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::G2], Tile::FIRST_PAWN);
        assert_eq!(result[Coordinate::H2], Tile::FIRST_PAWN);

        assert_eq!(result[Coordinate::A3], Tile::EMPTY);
        assert_eq!(result[Coordinate::B3], Tile::EMPTY);
        assert_eq!(result[Coordinate::C3], Tile::EMPTY);
        assert_eq!(result[Coordinate::D3], Tile::EMPTY);
        assert_eq!(result[Coordinate::E3], Tile::EMPTY);
        assert_eq!(result[Coordinate::F3], Tile::EMPTY);
        assert_eq!(result[Coordinate::G3], Tile::EMPTY);
        assert_eq!(result[Coordinate::H3], Tile::EMPTY);

        assert_eq!(result[Coordinate::A4], Tile::EMPTY);
        assert_eq!(result[Coordinate::B4], Tile::EMPTY);
        assert_eq!(result[Coordinate::C4], Tile::EMPTY);
        assert_eq!(result[Coordinate::D4], Tile::EMPTY);
        assert_eq!(result[Coordinate::E4], Tile::EMPTY);
        assert_eq!(result[Coordinate::F4], Tile::EMPTY);
        assert_eq!(result[Coordinate::G4], Tile::EMPTY);
        assert_eq!(result[Coordinate::H4], Tile::EMPTY);

        assert_eq!(result[Coordinate::A5], Tile::EMPTY);
        assert_eq!(result[Coordinate::B5], Tile::EMPTY);
        assert_eq!(result[Coordinate::C5], Tile::EMPTY);
        assert_eq!(result[Coordinate::D5], Tile::EMPTY);
        assert_eq!(result[Coordinate::E5], Tile::EMPTY);
        assert_eq!(result[Coordinate::F5], Tile::EMPTY);
        assert_eq!(result[Coordinate::G5], Tile::EMPTY);
        assert_eq!(result[Coordinate::H5], Tile::EMPTY);

        assert_eq!(result[Coordinate::A6], Tile::EMPTY);
        assert_eq!(result[Coordinate::B6], Tile::EMPTY);
        assert_eq!(result[Coordinate::C6], Tile::EMPTY);
        assert_eq!(result[Coordinate::D6], Tile::EMPTY);
        assert_eq!(result[Coordinate::E6], Tile::EMPTY);
        assert_eq!(result[Coordinate::F6], Tile::EMPTY);
        assert_eq!(result[Coordinate::G6], Tile::EMPTY);
        assert_eq!(result[Coordinate::H6], Tile::EMPTY);

        assert_eq!(result[Coordinate::A7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::B7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::C7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::D7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::E7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::F7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::G7], Tile::SECOND_PAWN);
        assert_eq!(result[Coordinate::H7], Tile::SECOND_PAWN);

        assert_eq!(result[Coordinate::A8], Tile::SECOND_ROOK);
        assert_eq!(result[Coordinate::B8], Tile::SECOND_KNIGHT);
        assert_eq!(result[Coordinate::C8], Tile::SECOND_BISHOP);
        assert_eq!(result[Coordinate::D8], Tile::SECOND_QUEEN);
        assert_eq!(result[Coordinate::E8], Tile::SECOND_KING);
        assert_eq!(result[Coordinate::F8], Tile::SECOND_BISHOP);
        assert_eq!(result[Coordinate::G8], Tile::SECOND_KNIGHT);
        assert_eq!(result[Coordinate::H8], Tile::SECOND_ROOK);
    }

    #[test]
    fn make_move_from_struct_not_san() {
        let state = GameState::new();
        let legal_move = PawnMove(Coordinate::E2, Coordinate::E4);

        let result = state.make_move(&legal_move);

        assert_that!(matches!(result, Some(_)))
    }

    #[test]
    fn given_move_not_in_possible_moves_returns_none() {
        let state = GameState::new();
        let illegal_move = PawnMove(Coordinate::A8, Coordinate::A6);

        let result = state.make_move(&illegal_move);

        assert_that!(matches!(result, None))
    }
}
