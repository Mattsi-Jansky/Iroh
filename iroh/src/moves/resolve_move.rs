use crate::state::coordinates::{File, Rank};
use crate::moves::Move;
use crate::state::GameState;
use crate::state::piece::{Piece, PieceType};

pub fn resolve_move(requested_move: &Move, mut game_state: GameState) -> GameState {
    let is_first_player_turn = game_state.is_first_player_turn;
    resolve_move_for(requested_move, game_state, is_first_player_turn)
}

pub fn resolve_move_for(requested_move: &Move, mut game_state: GameState, is_first_player: bool) -> GameState {
    match requested_move {
        Move::PawnMove(from, to_rank) => {
            move_piece(&mut game_state, &from.0, &from.1, &from.0, to_rank);
        }
        Move::RegularMove((from_file, from_rank), (to_file, to_rank), _) => {
            move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::AttackMove((from_file, from_rank), (to_file, to_rank), _) => {
            let piece = game_state.board[(to_file,to_rank)]
                .expect("Illegal move, no target to attack")
                .piece_type;
            if is_first_player {game_state.captured_pieces.captured_second_player(piece, game_state.turn_number);}
            else {game_state.captured_pieces.captured_first_player(piece, game_state.turn_number);}
            
            move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::PawnAttackMove(starting_file,(to_file, to_rank)) => {
            let piece = game_state.board[(to_file,to_rank)]
                .expect("Illegal move, no target to attack")
                .piece_type;
            if is_first_player {game_state.captured_pieces.captured_second_player(piece, game_state.turn_number);}
            else {game_state.captured_pieces.captured_first_player(piece, game_state.turn_number);}

            let direction = if is_first_player {-1} else {1};
            move_piece(&mut game_state,
                       starting_file,
                       &to_rank.transform(direction)
                           .expect("should not be possible to get an out of bounds pawn attack command"),
                       to_file,
                       to_rank);
        }
        Move::PawnPromotion(file, is_owned_by_first_player, piece_type) => {
            let from_rank = &Rank::new(if *is_owned_by_first_player {6} else {1});
            let to_rank = &Rank::new(if *is_owned_by_first_player {7} else {0});
            game_state.board[(file,from_rank)] = None;
            game_state.board[(file,to_rank)] =
                Some(Piece { is_owned_by_first_player: is_first_player, piece_type: *piece_type })
        }
        Move::Castle(is_kingside) => {
            match (is_first_player, *is_kingside) {
                (true, true) => {
                    move_piece(&mut game_state, &File::E, &Rank::ONE,
                               &File::G, &Rank::ONE);
                    move_piece(&mut game_state, &File::H, &Rank::ONE,
                               &File::F, &Rank::ONE);
                }
                (true, false) => {
                    move_piece(&mut game_state, &File::E, &Rank::ONE,
                               &File::C, &Rank::ONE);
                    move_piece(&mut game_state, &File::A, &Rank::ONE,
                               &File::D, &Rank::ONE);
                }
                (false, true) => {
                    move_piece(&mut game_state, &File::E, &Rank::EIGHT,
                               &File::G, &Rank::EIGHT);
                    move_piece(&mut game_state, &File::H, &Rank::EIGHT,
                               &File::F, &Rank::EIGHT);
                }
                (false, false) => {
                    move_piece(&mut game_state, &File::E, &Rank::EIGHT,
                               &File::C, &Rank::EIGHT);
                    move_piece(&mut game_state, &File::A, &Rank::EIGHT,
                               &File::D, &Rank::EIGHT);
                }
            }
        }
    }

    game_state.next_turn();
    game_state
}

fn move_piece(game_state: &mut GameState, from_file: &File, from_rank: &Rank, to_file: &File, to_rank: &Rank) {
    let from = (from_file, from_rank);
    let to = (to_file, to_rank);
    let piece = game_state.board[from];
    game_state.board[from] = None;
    game_state.board[to] = piece;

    update_castling_state(game_state, from, piece)
}

fn update_castling_state(game_state: &mut GameState, from: (&File, &Rank), piece: Option<Piece>) {
    let moving_piece_type = piece.unwrap().piece_type;
    if moving_piece_type == PieceType::Rook
        && from.0 == &File::H
        && from.1 == &Rank::ONE {
        game_state.first_player_can_castle_kingside = false;
    }
    else if moving_piece_type == PieceType::Rook
        && from.0 == &File::A
        && from.1 == &Rank::ONE {
        game_state.first_player_can_castle_queenside = false;
    }
    else if moving_piece_type == PieceType::King
        && from.0 == &File::E
        && from.1 == &Rank::ONE {
        game_state.first_player_can_castle_kingside = false;
        game_state.first_player_can_castle_queenside = false;
    }

    if moving_piece_type == PieceType::Rook
        && from.0 == &File::H
        && from.1 == &Rank::EIGHT {
        game_state.second_player_can_castle_kingside = false;
    }
    else if moving_piece_type == PieceType::Rook
        && from.0 == &File::A
        && from.1 == &Rank::EIGHT {
        game_state.second_player_can_castle_queenside = false;
    }
    else if moving_piece_type == PieceType::King
        && from.0 == &File::E
        && from.1 == &Rank::EIGHT {
        game_state.second_player_can_castle_kingside = false;
        game_state.second_player_can_castle_queenside = false;
    }
}
