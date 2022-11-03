use iroh::search::search;
use iroh::state::GameState;
use iroh::game::Game;

macro_rules! search_test_inner {
    ($name:ident,$fen:expr,$expected_san:expr) => {
        #[test]
        fn $name() {
            let state = Game::from_fen($fen);

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
    {gain_9_material_better_than_gaining_1,"8/k7/3q4/5p2/6P1/3Q4/K7/8 w - - 0 1","Qxd6"}
    {gain_9_material_better_than_gaining_5,"8/8/1rp5/4q3/3Q4/3R4/8/8 w - - 0 1","Qxe5"}
    {gain_3_material_better_than_gaining_1,"8/8/k2b2p1/8/8/K5Q1/8/8 w - - 0 1","Qxd6"}
    // {second_player_gain_9_material_better_than_gaining_5,"k7/8/8/3q4/2Q5/5R2/K7/8 b - - 0 1","Qxc4"}
    {checkmate_in_one,"3k4/7R/8/R7/8/8/8/3K4 w - - 0 1","Ra8"}
    // {discovered_check,"r3kb1r/ppq2ppp/2p5/3pN3/3P4/8/PPPQ1PPP/R3R1K1 w kq - 0 1","Ng6"}
}
