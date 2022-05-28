#[macro_export]
macro_rules! chess_test_inner {
    ($name:ident,[$($san:literal),+],$pgn:expr) => {
        #[test]
        fn $name() {
            let mut game = Game::new();

            $(game = game.make_move($san).unwrap();)+
            let result = game.get_pgn();

            assert_eq!($pgn, result);
        }
    };
    ($name:ident @ $fen:expr,[$($san:literal),+],$pgn:expr) => {
        #[test]
        fn $name() {
            let mut game = Game::from_fen($fen);

            $(game = game.make_move($san).unwrap();)+
            let result = game.get_pgn();

            assert_eq!($pgn, result);
        }
    };
    ($name:ident @ $fen:expr,[$($san:literal),+],$pgn:expr,$expected_fen:expr) => {
        #[test]
        fn $name() {
            let mut game = Game::from_fen($fen);

            $(game = game.make_move($san).unwrap();)+
            let result_pgn = game.get_pgn();
            let result_fen = game.generate_fen();

            assert_eq!($pgn, result_pgn);
            assert_eq!($expected_fen,result_fen);
        }
    }
}

#[macro_export]
macro_rules! chess_test {
    ($($myblock:tt)*) => {
        $(chess_test_inner! $myblock)*
    };
}

