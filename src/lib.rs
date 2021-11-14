pub struct ChessGame {
    sans: Vec<String>,
}

#[derive(Debug)]
pub struct ChessGameError {

}


impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame { sans: vec![] }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn get_pgn(&self) -> String {
        if self.sans.is_empty() {
            return String::new()
        }
        else {
            let mut result = String::new();
            let mut i = 1;
            for pair in self.sans.chunks(2) {
                let turn = format!("{}. {}", i, pair[0]);
                if pair.len() > 1 {
                    result = format!("{}{} {} {}", result, turn, pair[1], if i != (self.sans.len() as f32 / 2 as f32).ceil() as u8 {""} else {"*"})
                } else {
                    result = format!("{}{} *", result, turn)
                }
                i = i + 1;
            }
            return result
        }
    }

    pub fn make_move(self, san: &str) -> Result<ChessGame,ChessGameError> {

        // let column = &(san.chars().nth(0).unwrap() as u32) - 97;
        // let row = &san.chars().nth(1).unwrap().to_digit(10).unwrap();
        let mut new = self.sans.clone();
        new.push(String::from(san));

        Ok(ChessGame { sans: new })
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Move {
    RegularMove((u8, u8), (u8, u8))
}
