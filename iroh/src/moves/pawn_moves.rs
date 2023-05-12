use crate::moves::Move;
use crate::state::coordinates::Coordinate;
use crate::state::tile::Tile;
use crate::state::GameState;

pub fn generate_pawn_moves(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    pawn: (Tile, Coordinate),
    is_for_first_player: bool,
) {
    let ahead_coordinate = (if is_for_first_player {
        pawn.1.north()
    } else {
        pawn.1.south()
    })
    .expect("Pawn is never on back rank, because of promotion");

    if !game_state.board[ahead_coordinate].is_occupied() {
        let ahead_rank_is_last_rank = if is_for_first_player {
            ahead_coordinate.is_last_rank()
        } else {
            ahead_coordinate.is_first_rank()
        };
        let is_on_starting_rank = if is_for_first_player {
            pawn.1.is_rank_2()
        } else {
            pawn.1.is_rank_7()
        };

        if ahead_rank_is_last_rank {
            generate_promotion_moves(available_moves, ahead_coordinate, is_for_first_player);
        } else {
            available_moves.push(Move::PawnMove(pawn.1, ahead_coordinate));

            if is_on_starting_rank {
                generate_double_move(
                    game_state,
                    available_moves,
                    pawn,
                    ahead_coordinate,
                    is_for_first_player,
                )
            }
        }
    }
    generate_attack_moves(game_state, available_moves, pawn, is_for_first_player);
    generate_en_passant(game_state, available_moves, pawn, is_for_first_player);
}

///**Invariant**: Only call function if pawn is on their starting rank
fn generate_double_move(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    pawn: (Tile, Coordinate),
    ahead_coordinate: Coordinate,
    is_for_first_player: bool,
) {
    let ahead_twice_coordinate = (if is_for_first_player {
        ahead_coordinate.north()
    } else {
        ahead_coordinate.south()
    })
    .expect("Invariant breached - pawn should be on starting rank");
    if !game_state.board[ahead_twice_coordinate].is_occupied() {
        available_moves.push(Move::PawnMove(pawn.1, ahead_twice_coordinate));
    }
}

fn generate_attack_moves(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    pawn: (Tile, Coordinate),
    is_for_first_player: bool,
) {
    let west_target = if is_for_first_player {
        pawn.1.north_west()
    } else {
        pawn.1.south_west()
    };
    let east_target = if is_for_first_player {
        pawn.1.north_east()
    } else {
        pawn.1.south_east()
    };

    generate_attack_move(
        game_state,
        available_moves,
        pawn.1,
        west_target,
        is_for_first_player,
    );
    generate_attack_move(
        game_state,
        available_moves,
        pawn.1,
        east_target,
        is_for_first_player,
    );
}

fn generate_attack_move(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    start: Coordinate,
    target: Option<Coordinate>,
    is_for_first_player: bool,
) {
    if let Some(target) = target {
        let tile = game_state.board[target];
        if tile.is_occupied() && tile.is_owned_by_first_player() != is_for_first_player {
            available_moves.push(Move::PawnAttackMove(start, target))
        }
    }
}

fn generate_promotion_moves(
    available_moves: &mut Vec<Move>,
    to: Coordinate,
    is_for_first_player: bool,
) {
    available_moves.push(Move::PawnPromotion(
        to,
        if is_for_first_player {
            Tile::FIRST_QUEEN
        } else {
            Tile::SECOND_QUEEN
        },
    ));
    available_moves.push(Move::PawnPromotion(
        to,
        if is_for_first_player {
            Tile::FIRST_ROOK
        } else {
            Tile::SECOND_ROOK
        },
    ));
    available_moves.push(Move::PawnPromotion(
        to,
        if is_for_first_player {
            Tile::FIRST_BISHOP
        } else {
            Tile::SECOND_BISHOP
        },
    ));
    available_moves.push(Move::PawnPromotion(
        to,
        if is_for_first_player {
            Tile::FIRST_KNIGHT
        } else {
            Tile::SECOND_KNIGHT
        },
    ));
}

fn generate_en_passant(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    pawn: (Tile, Coordinate),
    is_for_first_player: bool,
) {
    if is_for_first_player && pawn.1.is_rank_5() {
        let move_target = pawn.1.north_east();
        let attack_target = pawn.1.east();
        if let (Some(move_target), Some(attack_target)) = (move_target, attack_target) {
            if game_state.sans.last() == Some(&format!("{}",attack_target)) {
                available_moves.push(Move::EnPassant(pawn.1, move_target))
            }
        }
    } else if !is_for_first_player && pawn.1.is_rank_4() {
        let move_target = pawn.1.south_west();
        let attack_target = pawn.1.west();
        if let (Some(move_target), Some(attack_target)) = (move_target, attack_target) {
            if game_state.board[attack_target] == Tile::FIRST_PAWN {
                available_moves.push(Move::EnPassant(pawn.1, move_target))
            }
        }
    }
}
