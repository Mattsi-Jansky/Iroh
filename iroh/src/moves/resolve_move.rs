use crate::state::coordinates::{File, Rank};
use crate::moves::Move;
use crate::state::GameState;
use crate::state::piece::{Piece, PieceType};

pub fn resolve_move(requested_move: Move, mut game_state: GameState) -> GameState {
    match requested_move {
        Move::PawnMove(from, to_rank) => {
            move_piece(&mut game_state, from.0, from.1, from.0, to_rank);
        }
        Move::RegularMove((from_file, from_rank), (to_file, to_rank), _) => {
            move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::AttackMove((from_file, from_rank), (to_file, to_rank), _) => {
            let piece = game_state.board[(to_file,to_rank)]
                .expect("Illegal move, no target to attack")
                .piece_type;
            if game_state.is_first_player_turn() {game_state.captured_pieces.second_player.push(piece);}
            else {game_state.captured_pieces.first_player.push(piece);}
            
            move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::PawnAttackMove(starting_file,(to_file, to_rank)) => {
            let piece = game_state.board[(to_file,to_rank)]
                .expect("Illegal move, no target to attack")
                .piece_type;
            if game_state.is_first_player_turn() {game_state.captured_pieces.second_player.push(piece);}
            else {game_state.captured_pieces.first_player.push(piece);}

            let direction = if game_state.is_first_player_turn() {-1} else {1};
            move_piece(&mut game_state,
                       starting_file,
                       to_rank.transform(direction)
                           .expect("should not be possible to get an out of bounds pawn attack command"),
                       to_file,
                       to_rank);
        }
        Move::PawnPromotion(file, is_owned_by_first_player, piece_type) => {
            let from_rank = Rank::new(if is_owned_by_first_player {6} else {1});
            let to_rank = Rank::new(if is_owned_by_first_player {7} else {0});
            game_state.board[(file,from_rank)] = None;
            game_state.board[(file,to_rank)] =
                Some(Piece { is_owned_by_first_player: game_state.is_first_player_turn(), piece_type })
        }
        Move::Castle(is_kingside) => {
            if game_state.is_first_player_turn() {
                if is_kingside {
                    move_piece(&mut game_state, File::new(4), Rank::new(0),
                               File::new(6), Rank::new(0));
                    move_piece(&mut game_state, File::new(7), Rank::new(0),
                               File::new(5), Rank::new(0));
                } else {
                    move_piece(&mut game_state, File::new(4), Rank::new(0),
                               File::new(2), Rank::new(0));
                    move_piece(&mut game_state, File::new(0), Rank::new(0),
                               File::new(3), Rank::new(0));
                }
            } else {
                if is_kingside {
                    move_piece(&mut game_state, File::new(4), Rank::new(7),
                               File::new(6), Rank::new(7));
                    move_piece(&mut game_state, File::new(7), Rank::new(7),
                               File::new(5), Rank::new(7));
                } else {
                    move_piece(&mut game_state, File::new(4), Rank::new(7),
                               File::new(2), Rank::new(7));
                    move_piece(&mut game_state, File::new(0), Rank::new(7),
                               File::new(3), Rank::new(7));
                }
            }
        }
    }

    game_state.increment_turn_number();
    game_state
}

fn move_piece(game_state: &mut GameState, from_file: File, from_rank: Rank, to_file: File, to_rank: Rank) {
    let from = (from_file, from_rank);
    let to = (to_file, to_rank);
    let piece = game_state.board[from];
    game_state.board[from] = None;
    game_state.board[to] = piece;

    update_castling_state(game_state, from, piece)
}

fn update_castling_state(game_state: &mut GameState, from: (File, Rank), piece: Option<Piece>) {
    let moving_piece_type = piece.unwrap().piece_type;
    if moving_piece_type == PieceType::Rook
        && from.0 == File::new(7)
        && from.1 == Rank::new(0) {
        game_state.first_player_can_castle_kingside = false;
    }
    else if moving_piece_type == PieceType::Rook
        && from.0 == File::new(0)
        && from.1 == Rank::new(0) {
        game_state.first_player_can_castle_queenside = false;
    }
    else if moving_piece_type == PieceType::King
        && from.0 == File::new(4)
        && from.1 == Rank::new(0) {
        game_state.first_player_can_castle_kingside = false;
        game_state.first_player_can_castle_queenside = false;
    }

    if moving_piece_type == PieceType::Rook
        && from.0 == File::new(7)
        && from.1 == Rank::new(7) {
        game_state.second_player_can_castle_kingside = false;
    }
    else if moving_piece_type == PieceType::Rook
        && from.0 == File::new(0)
        && from.1 == Rank::new(7) {
        // game_state.first_player_can_castle_queenside = false;
    }
    else if moving_piece_type == PieceType::King
        && from.0 == File::new(4)
        && from.1 == Rank::new(7) {
        game_state.second_player_can_castle_kingside = false;
        game_state.second_player_can_castle_queenside = false;
    }
}
