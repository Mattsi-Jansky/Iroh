use iroh::evaluations::evaluate;
use iroh::state::GameState;

macro_rules! evaluation_test_inner {
    ($name:ident,$fen:expr,$expected_san:expr) => {
        #[test]
        fn $name() {
            let state = GameState::from_fen($fen);

            let result = evaluate(&state);

            assert_eq!(String::from($expected_san), result.best_move);
        }
    }
}

macro_rules! evaluation_tests {
    ($($myblock:tt)*) => {
        $(evaluation_test_inner! $myblock)*
    };
}

evaluation_tests! {
    {simple_puzzle,"3k4/8/8/3p4/4P3/8/8/3K4 w - - 0 1","exd5"}
}
