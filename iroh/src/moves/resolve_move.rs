use crate::moves::Move;
use crate::state::coordinates::Coordinate;
use crate::state::tile::Tile;
use crate::state::GameState;

pub struct ResolvedMoveMemento<'a> {
    last_move: &'a Move,
    captured_piece: Tile,
    is_first_player: bool, //TODO So long as we only go 1 level deep we can calculate this
    castling_state: CastlingStateMemento,
}

impl<'a> ResolvedMoveMemento<'a> {
    fn new(
        last_move: &'a Move,
        captured_piece: Tile,
        is_first_player: bool,
        castling_state: CastlingStateMemento,
    ) -> ResolvedMoveMemento<'a> {
        ResolvedMoveMemento {
            last_move,
            captured_piece,
            is_first_player,
            castling_state,
        }
    }
}

struct CastlingStateMemento {
    first_player_can_castle_kingside: bool,
    first_player_can_castle_queenside: bool,
    second_player_can_castle_kingside: bool,
    second_player_can_castle_queenside: bool,
}

impl CastlingStateMemento {
    fn new(game_state: &GameState) -> CastlingStateMemento {
        CastlingStateMemento {
            first_player_can_castle_kingside: game_state.first_player_can_castle_kingside,
            first_player_can_castle_queenside: game_state.first_player_can_castle_queenside,
            second_player_can_castle_kingside: game_state.second_player_can_castle_kingside,
            second_player_can_castle_queenside: game_state.second_player_can_castle_queenside,
        }
    }

    pub fn apply(&self, game_state: &mut GameState) {
        game_state.first_player_can_castle_kingside = self.first_player_can_castle_kingside;
        game_state.first_player_can_castle_queenside = self.first_player_can_castle_queenside;
        game_state.second_player_can_castle_kingside = self.second_player_can_castle_kingside;
        game_state.second_player_can_castle_queenside = self.second_player_can_castle_queenside;
    }
}

pub fn resolve_move<'a>(requested_move: &'a Move, game_state: &mut GameState) -> ResolvedMoveMemento<'a> {
    let is_first_player_turn = game_state.is_first_player_turn;
    let memento = perform_move_for(requested_move, game_state, is_first_player_turn);
    if memento.captured_piece != Tile::EMPTY {
        if is_first_player_turn {
            game_state
                .captured_pieces
                .captured_second_player(memento.captured_piece, game_state.turn_number);
        } else {
            game_state
                .captured_pieces
                .captured_first_player(memento.captured_piece, game_state.turn_number);
        }
    }
    game_state.next_turn();
    memento
}

pub fn perform_move_for<'a>(
    requested_move: &'a Move,
    game_state: &mut GameState,
    is_first_player: bool,
) -> ResolvedMoveMemento<'a> {
    let castle_state = CastlingStateMemento::new(game_state);

    match requested_move {
        Move::PawnMove(from, to) => {
            move_piece(game_state, from, to);
            ResolvedMoveMemento::new(requested_move, Tile::EMPTY, is_first_player, castle_state)
        }
        Move::RegularMove(from, to, _) => {
            move_piece(game_state, from, to);
            ResolvedMoveMemento::new(requested_move, Tile::EMPTY, is_first_player, castle_state)
        }
        Move::AttackMove(from, to, _) => {
            let target_tile = game_state.board[to];
            assert!(
                target_tile.is_occupied(),
                "Illegal move, no target to attack"
            );

            move_piece(game_state, from, to);
            ResolvedMoveMemento::new(requested_move, target_tile, is_first_player, castle_state)
        }
        Move::PawnAttackMove(from, to) => {
            let target_tile = game_state.board[to];
            assert!(
                target_tile.is_occupied(),
                "Illegal move, no target to attack"
            );

            move_piece(game_state, from, to);
            ResolvedMoveMemento::new(requested_move, target_tile, is_first_player, castle_state)
        }
        Move::PawnPromotion(target, tile) => {
            let from = (if tile.is_owned_by_first_player() {
                target.south()
            } else {
                target.north()
            })
            .expect("Cannot resolve pawn promotion, given invalid move");
            game_state.board[from] = Tile::EMPTY;
            game_state.board[target] = *tile;
            ResolvedMoveMemento::new(requested_move, Tile::EMPTY, is_first_player, castle_state)
        }
        Move::Castle(is_kingside) => {
            match (is_first_player, is_kingside) {
                (true, true) => {
                    move_piece(game_state, &Coordinate::E1, &Coordinate::G1);
                    move_piece(game_state, &Coordinate::H1, &Coordinate::F1);
                }
                (true, false) => {
                    move_piece(game_state, &Coordinate::E1, &Coordinate::C1);
                    move_piece(game_state, &Coordinate::A1, &Coordinate::D1);
                }
                (false, true) => {
                    move_piece(game_state, &Coordinate::E8, &Coordinate::G8);
                    move_piece(game_state, &Coordinate::H8, &Coordinate::F8);
                }
                (false, false) => {
                    move_piece(game_state, &Coordinate::E8, &Coordinate::C8);
                    move_piece(game_state, &Coordinate::A8, &Coordinate::D8);
                }
            }
            ResolvedMoveMemento::new(requested_move, Tile::EMPTY, is_first_player, castle_state)
        },
        Move::EnPassant(from,to) => {
            move_piece(game_state, from, to);
            game_state.board[to.south().unwrap()] = Tile::EMPTY;
            ResolvedMoveMemento::new(requested_move, Tile::SECOND_PAWN, is_first_player, castle_state)
        }
    }
}

pub fn undo_move(memento: ResolvedMoveMemento, game_state: &mut GameState) {
    let ResolvedMoveMemento {
        last_move,
        captured_piece,
        is_first_player,
        castling_state,
    } = memento;

    match last_move {
        Move::RegularMove(from, to, _) => {
            move_piece(game_state, to, from);
        }
        Move::AttackMove(from, to, _) => {
            move_piece(game_state, to, from);
            game_state.board[to] = captured_piece;
        }
        Move::PawnMove(from, to) => {
            move_piece(game_state, to, from);
        }
        Move::PawnAttackMove(from, to) => {
            move_piece(game_state, to, from);
            game_state.board[to] = captured_piece;
        }
        Move::PawnPromotion(to, _) => {
            game_state.board[to] = Tile::EMPTY;
            let from = if is_first_player {
                to.south().unwrap()
            } else {
                to.north().unwrap()
            };
            game_state.board[from] = if is_first_player {
                Tile::FIRST_PAWN
            } else {
                Tile::SECOND_PAWN
            };
        }
        Move::Castle(is_kingside) => match (is_first_player, is_kingside) {
            (true, true) => {
                move_piece(game_state, &Coordinate::G1, &Coordinate::E1);
                move_piece(game_state, &Coordinate::F1, &Coordinate::H1);
            }
            (true, false) => {
                move_piece(game_state, &Coordinate::C1, &Coordinate::E1);
                move_piece(game_state, &Coordinate::D1, &Coordinate::A1);
            }
            (false, true) => {
                move_piece(game_state, &Coordinate::G8, &Coordinate::E8);
                move_piece(game_state, &Coordinate::F8, &Coordinate::H8);
            }
            (false, false) => {
                move_piece(game_state, &Coordinate::C8, &Coordinate::E8);
                move_piece(game_state, &Coordinate::D8, &Coordinate::A8);
            }
        },
        Move::EnPassant(from,to) => {
            move_piece(game_state, to, from);
            if is_first_player {
                game_state.board[to.south().unwrap()] = memento.captured_piece;
            } else {
                game_state.board[to.north().unwrap()] = memento.captured_piece;
            }
        }
    }
    castling_state.apply(game_state);
}

fn move_piece(game_state: &mut GameState, from: &Coordinate, to: &Coordinate) {
    let tile = game_state.board[from];
    game_state.board[from] = Tile::EMPTY;
    game_state.board[to] = tile;
    update_castling_state(game_state, from, tile);
}

fn update_castling_state(game_state: &mut GameState, from: &Coordinate, tile: Tile) {
    if tile == Tile::FIRST_ROOK && from == &Coordinate::H1 {
        game_state.first_player_can_castle_kingside = false;
    } else if tile == Tile::FIRST_ROOK && from == &Coordinate::A1 {
        game_state.first_player_can_castle_queenside = false;
    } else if tile == Tile::FIRST_KING && from == &Coordinate::E1 {
        game_state.first_player_can_castle_kingside = false;
        game_state.first_player_can_castle_queenside = false;
    }

    if tile == Tile::SECOND_ROOK && from == &Coordinate::H8 {
        game_state.second_player_can_castle_kingside = false;
    } else if tile == Tile::SECOND_ROOK && from == &Coordinate::A8 {
        game_state.second_player_can_castle_queenside = false;
    } else if tile == Tile::SECOND_KING && from == &Coordinate::E8 {
        game_state.second_player_can_castle_kingside = false;
        game_state.second_player_can_castle_queenside = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::Move::*;

    #[test]
    fn undo_regular_move() {
        let mut state =
            GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let requested_move = RegularMove(Coordinate::G1, Coordinate::F3, Tile::FIRST_KNIGHT);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
            state.generate_fen()
        );

        undo_move(memento, &mut state);

        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            state.generate_fen()
        );
    }

    #[test]
    fn undo_attack_move() {
        let mut state = GameState::from_fen("3k4/8/8/8/2p5/3K4/8/8 w - - 0 1");
        let requested_move = AttackMove(Coordinate::D3, Coordinate::C4, Tile::FIRST_KING);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("3k4/8/8/8/2K5/8/8/8 w - - 0 1", state.generate_fen());
        assert_eq!(state.captured_pieces.second_player.len(), 0);

        undo_move(memento, &mut state);

        assert_eq!("3k4/8/8/8/2p5/3K4/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_pawn_move() {
        let mut state = GameState::from_fen("4k3/8/8/2P5/1K6/8/8/8 w - - 0 1");
        let requested_move = PawnMove(Coordinate::C5, Coordinate::C6);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("4k3/8/2P5/8/1K6/8/8/8 w - - 0 1", state.generate_fen());

        undo_move(memento, &mut state);

        assert_eq!("4k3/8/8/2P5/1K6/8/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_pawn_attack() {
        let mut state = GameState::from_fen("4k3/8/3p4/2P5/1K6/8/8/8 w - - 0 1");
        let requested_move = PawnAttackMove(Coordinate::C5, Coordinate::D6);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("4k3/8/3P4/8/1K6/8/8/8 w - - 0 1", state.generate_fen());

        undo_move(memento, &mut state);

        assert_eq!("4k3/8/3p4/2P5/1K6/8/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_pawn_promotion() {
        let mut state = GameState::from_fen("6k1/2P5/8/8/1K6/8/8/8 w - - 0 1");
        let requested_move = PawnPromotion(Coordinate::C8, Tile::FIRST_QUEEN);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("2Q3k1/8/8/8/1K6/8/8/8 w - - 0 1", state.generate_fen());

        undo_move(memento, &mut state);

        assert_eq!("6k1/2P5/8/8/1K6/8/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_castling() {
        let mut state = GameState::from_fen("4k3/8/8/8/8/8/8/4K2R w K - 0 1");
        let requested_move = Castle(true);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("4k3/8/8/8/8/8/8/5RK1 w - - 0 1", state.generate_fen());

        undo_move(memento, &mut state);

        assert_eq!("4k3/8/8/8/8/8/8/4K2R w K - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_en_passant() {
        let mut state = GameState::from_fen("3k4/2p5/8/1P6/8/8/8/3K4 b - - 0 1");
        let mut state = state.make_move_san("c5").unwrap();
        assert_eq!("3k4/8/8/1Pp5/8/8/8/3K4 w - - 0 1", state.generate_fen());
        let requested_move = EnPassant(Coordinate::B5, Coordinate::C6);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("3k4/8/2P5/8/8/8/8/3K4 w - - 0 1", state.generate_fen());

        undo_move(memento, &mut state);

        assert_eq!("3k4/8/8/1Pp5/8/8/8/3K4 w - - 0 1", state.generate_fen());
    }
}
