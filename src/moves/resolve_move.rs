use crate::state::board::Board;
use crate::state::coordinates::{File, Rank};
use crate::moves::Move;
use crate::state::GameState;

pub fn resolve_move(requested_move: Move, mut game_state: GameState) -> GameState {
    match requested_move {
        Move::PawnMove(from_file, (to_file, to_rank)) => {
            if from_file == to_file {
                move_piece(&mut game_state.board, from_file, to_rank - 1, to_file, to_rank);
            } else { todo!("Pawn attack moves")}
        }
        Move::RegularMove((from_file, from_rank), (to_file, to_rank), _) => {
            move_piece(&mut game_state.board, from_file, from_rank, to_file, to_rank);
        }
        Move::AttackMove((from_file, from_rank), (to_file, to_rank), _) => {
            let piece = game_state.board[(to_file,to_rank)]
                .expect("Illegal move, no target to attack")
                .piece_type;
            if game_state.is_first_player_turn() {game_state.captured_pieces.second_player.push(piece);}
            else {game_state.captured_pieces.first_player.push(piece);}
            move_piece(&mut game_state.board, from_file, from_rank, to_file, to_rank);
        }
    }

    game_state.increment_turn_number();
    game_state
}

fn move_piece(board: &mut Board, from_file: File, from_rank: Rank, to_file: File, to_rank: Rank) {
    let from = (from_file, from_rank);
    let to = (to_file, to_rank);
    let piece = board[from];
    board[from] = None;
    board[to] = piece;
}
