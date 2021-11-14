pub struct ChessGame {
    pgn: String
}

#[derive(Debug)]
pub struct ChessGameError {

}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame { pgn: String::new() }
    }

    pub fn get_pgn(&self) -> String {
        String::from(&self.pgn[..])
    }

    pub fn make_move(self, san: &str) -> Result<ChessGame,ChessGameError> {
        Ok(ChessGame { pgn: String::from("1. d4 *") })
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
