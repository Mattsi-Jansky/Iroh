use crate::state::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedPieces {
    pub first_player: Vec<Tile>,
    pub second_player: Vec<Tile>,
    pub last_capture_turn: u16,
}

impl CapturedPieces {
    pub fn new() -> CapturedPieces {
        CapturedPieces {
            first_player: vec![],
            second_player: vec![],
            last_capture_turn: 0,
        }
    }

    pub fn captured_first_player(&mut self, tile: Tile, turn_number: u16) {
        self.first_player.push(tile);
        self.last_capture_turn = turn_number;
    }

    pub fn captured_second_player(&mut self, tile: Tile, turn_number: u16) {
        self.second_player.push(tile);
        self.last_capture_turn = turn_number;
    }
}

impl Default for CapturedPieces {
    fn default() -> Self {
        CapturedPieces::new()
    }
}
