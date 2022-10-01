#[macro_export]
macro_rules! chess_test_inner {
    ($name:ident,$sans:tt,$pgn:expr) => {
        chess_test_inner! { $name @ "",$sans,$pgn,"" }
    };
    ($name:ident @ $fen:expr,$sans:tt,$pgn:expr) => {
        chess_test_inner! {$name @ $fen,$sans,$pgn,""}
    };
    ($name:ident @ $fen:expr,[$($san:literal),+],$pgn:expr,$expected_fen:expr) => {
        #[test]
        fn $name() {
            let mut game = if $fen.is_empty() {Game::new()} else {GameInner::from_fen($fen)};

            $(game = game.unwrap().make_move($san);)+
            let result_pgn = game.generate_pgn().unwrap();
            let result_fen = game.generate_fen().unwrap();

            assert_eq!($pgn, result_pgn);
            if !$expected_fen.is_empty() {assert_eq!($expected_fen,result_fen)}
        }
    }
}

#[macro_export]
macro_rules! chess_test {
    ($($myblock:tt)*) => {
        $(chess_test_inner! $myblock)*
    };
}
