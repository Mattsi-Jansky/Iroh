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

    pub fn get_pgn(&self) -> String {
        if self.sans.len() == 0 {
            String::new()
        }
        else {
            String::from(format!("1. {} *", self.sans[0]))
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
