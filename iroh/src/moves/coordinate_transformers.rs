use crate::state::coordinates::Coordinate;

pub const STRAIGHT_DYNAMIC_TRANSFORMERS: [fn(Coordinate) -> Option<Coordinate>; 4] =
    [|c| c.north(), |c| c.east(), |c| c.south(), |c| c.west()];

pub const DIAGONAL_DYNAMIC_TRANSFORMERS: [fn(Coordinate) -> Option<Coordinate>; 4] = [
    |c| c.north_east(),
    |c| c.north_west(),
    |c| c.south_east(),
    |c| c.south_west(),
];

pub const STRAIGHT_AND_DIAGONAL_TRANSFORMERS: [fn(Coordinate) -> Option<Coordinate>; 8] = [
    |c| c.north(),
    |c| c.east(),
    |c| c.south(),
    |c| c.west(),
    |c| c.north_east(),
    |c| c.north_west(),
    |c| c.south_east(),
    |c| c.south_west(),
];

pub const KING_STATIC_TRANSFORMERS: [fn(Coordinate) -> Option<Coordinate>; 8] = [
    |c| c.north(),
    |c| c.east(),
    |c| c.south(),
    |c| c.west(),
    |c| c.north_east(),
    |c| c.north_west(),
    |c| c.south_east(),
    |c| c.south_west(),
];

pub const KNIGHT_STATIC_TRANSFORMERS: [fn(Coordinate) -> Option<Coordinate>; 8] = [
    |c| c.east().and_then(|c| c.east()).and_then(|c| c.north()),
    |c| c.east().and_then(|c| c.north()).and_then(|c| c.north()),
    |c| c.west().and_then(|c| c.south()).and_then(|c| c.south()),
    |c| c.west().and_then(|c| c.west()).and_then(|c| c.south()),
    |c| c.east().and_then(|c| c.south()).and_then(|c| c.south()),
    |c| c.east().and_then(|c| c.east()).and_then(|c| c.south()),
    |c| c.west().and_then(|c| c.north()).and_then(|c| c.north()),
    |c| c.west().and_then(|c| c.west()).and_then(|c| c.north()),
];

pub const PAWN_STATIC_TRANSFORMERS_FIRST_PLAYER: [fn(Coordinate) -> Option<Coordinate>; 2] =
    [|c| c.north_east(), |c| c.north_west()];

pub const PAWN_STATIC_TRANSFORMERS_SECOND_PLAYER: [fn(Coordinate) -> Option<Coordinate>; 2] =
    [|c| c.south_east(), |c| c.south_west()];
