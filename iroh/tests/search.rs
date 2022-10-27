use iroh::search::search;
use iroh::state::GameState;

macro_rules! search_test_inner {
    ($name:ident,$fen:expr,$expected_san:expr) => {
        #[test]
        fn $name() {
            let state = GameState::from_fen($fen);

            let result = search(&state);

            assert_eq!(String::from($expected_san), result.best_move);
        }
    }
}

macro_rules! search_tests {
    ($($myblock:tt)*) => {
        $(search_test_inner! $myblock)*
    };
}

search_tests! {
    {gain_1_material,"3k4/8/8/3p4/4P3/8/8/3K4 w - - 0 1","exd5"}
    {gain_3_material_knight,"3k4/8/3n4/8/4N3/8/8/3K4 w - - 0 1","Nxd6"}
    {gain_3_material_bishop,"3k4/8/2b5/8/4B3/8/8/3K4 w - - 0 1","Bxc6"}
    {gain_5_material,"3k4/8/3r4/8/8/3R4/8/3K4 w - - 0 1","Rxd6"}
    {gain_9_material,"3k4/8/3q4/8/8/3Q4/8/3K4 w - - 0 1","Qxd6"}
    {gain_9_material_better_than_gaining_1,"k7/8/1r6/4q3/3Q4/3R4/K7/8 w - - 0 1","Qxe5"}
}
