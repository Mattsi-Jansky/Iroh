pub struct ChessGame {
    sans: Vec<String>,
}

#[derive(Debug)]
pub struct ChessGameError {

}

fn generate_pgn_chunk(number_of_pairs: u8, mut result: String, index: u8, pair: &[String]) -> String{
    let turn = format!("{}. {}", index, pair[0]);
    if pair.len() > 1 {
        let asterisk_if_end = if index != number_of_pairs {
            ""
        } else {
            "*"
        };
        result = format!("{}{} {} {}", result, turn, pair[1], asterisk_if_end)
    } else {
        result = format!("{}{} *", result, turn)
    }
    result
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
            let number_of_pairs = (self.sans.len() as f32 / 2 as f32).ceil() as u8;
            let mut result = String::new();
            let mut i = 1;
            for pair in self.sans.chunks(2) {
                result = generate_pgn_chunk(number_of_pairs, result, i, pair);
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
