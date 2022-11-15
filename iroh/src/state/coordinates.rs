use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use derive_more::{Deref, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Hash)]
pub struct Coordinate(u8);

const SAN_LOOKUP: [&str; 8*8] = [
    "a1","b1","c1","d1","e1","f1","g1","h1",
    "a2","b2","c2","d2","e2","f2","g2","h2",
    "a3","b3","c3","d3","e3","f3","g3","h3",
    "a4","b4","c4","d4","e4","f4","g4","h4",
    "a5","b5","c5","d5","e5","f5","g5","h5",
    "a6","b6","c6","d6","e6","f6","g6","h6",
    "a7","b7","c7","d7","e7","f7","g7","h7",
    "a8","b8","c8","d8","e8","f8","g8","h8",
];
const MAX_INDEX: u8 = 8*8;

impl Coordinate {
    pub const A1: Coordinate = Coordinate(0);
    pub const B1: Coordinate = Coordinate(1);
    pub const C1: Coordinate = Coordinate(2);
    pub const D1: Coordinate = Coordinate(3);
    pub const E1: Coordinate = Coordinate(4);
    pub const F1: Coordinate = Coordinate(5);
    pub const G1: Coordinate = Coordinate(6);
    pub const H1: Coordinate = Coordinate(7);

    pub const A2: Coordinate = Coordinate(16);
    pub const B2: Coordinate = Coordinate(17);
    pub const C2: Coordinate = Coordinate(18);
    pub const D2: Coordinate = Coordinate(19);
    pub const E2: Coordinate = Coordinate(20);
    pub const F2: Coordinate = Coordinate(21);
    pub const G2: Coordinate = Coordinate(22);
    pub const H2: Coordinate = Coordinate(23);

    pub const A3: Coordinate = Coordinate(32);
    pub const B3: Coordinate = Coordinate(33);
    pub const C3: Coordinate = Coordinate(34);
    pub const D3: Coordinate = Coordinate(35);
    pub const E3: Coordinate = Coordinate(36);
    pub const F3: Coordinate = Coordinate(37);
    pub const G3: Coordinate = Coordinate(38);
    pub const H3: Coordinate = Coordinate(39);

    pub const A4: Coordinate = Coordinate(48);
    pub const B4: Coordinate = Coordinate(49);
    pub const C4: Coordinate = Coordinate(50);
    pub const D4: Coordinate = Coordinate(51);
    pub const E4: Coordinate = Coordinate(52);
    pub const F4: Coordinate = Coordinate(53);
    pub const G4: Coordinate = Coordinate(54);
    pub const H4: Coordinate = Coordinate(55);

    pub const A5: Coordinate = Coordinate(64);
    pub const B5: Coordinate = Coordinate(65);
    pub const C5: Coordinate = Coordinate(66);
    pub const D5: Coordinate = Coordinate(67);
    pub const E5: Coordinate = Coordinate(68);
    pub const F5: Coordinate = Coordinate(69);
    pub const G5: Coordinate = Coordinate(70);
    pub const H5: Coordinate = Coordinate(71);

    pub const A6: Coordinate = Coordinate(80);
    pub const B6: Coordinate = Coordinate(81);
    pub const C6: Coordinate = Coordinate(82);
    pub const D6: Coordinate = Coordinate(83);
    pub const E6: Coordinate = Coordinate(84);
    pub const F6: Coordinate = Coordinate(85);
    pub const G6: Coordinate = Coordinate(86);
    pub const H6: Coordinate = Coordinate(87);

    pub const A7: Coordinate = Coordinate(96);
    pub const B7: Coordinate = Coordinate(97);
    pub const C7: Coordinate = Coordinate(98);
    pub const D7: Coordinate = Coordinate(99);
    pub const E7: Coordinate = Coordinate(100);
    pub const F7: Coordinate = Coordinate(101);
    pub const G7: Coordinate = Coordinate(102);
    pub const H7: Coordinate = Coordinate(103);

    pub const A8: Coordinate = Coordinate(112);
    pub const B8: Coordinate = Coordinate(113);
    pub const C8: Coordinate = Coordinate(114);
    pub const D8: Coordinate = Coordinate(115);
    pub const E8: Coordinate = Coordinate(116);
    pub const F8: Coordinate = Coordinate(117);
    pub const G8: Coordinate = Coordinate(118);
    pub const H8: Coordinate = Coordinate(119);

    /// **Invariant:** input must be below 64
    /// Giving input higher than 64 will crash your program for certain.
    /// Only use when you are sure of the input size.
    pub fn from_usize_no_bounds_check(input: usize) -> Self {
        Coordinate(input as u8)
    }

    /// **Invariant:** input must be below 64
    /// Giving input higher than 64 will crash your program for certain.
    /// Only use when you are sure of the input size.
    pub fn from_u8_no_bounds_check(input: u8) -> Self {
        Coordinate(input)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn file(&self) -> &str {
        &SAN_LOOKUP[self.0 as usize][0..1]
    }

    pub fn north(&self) -> Option<Coordinate> {
        self.checked_add(16)
    }

    pub fn east(&self) -> Option<Coordinate> {
        self.checked_add(1)
    }

    pub fn south(&self) -> Option<Coordinate> {
        self.checked_sub(16)
    }

    pub fn west(&self) -> Option<Coordinate> {
        self.checked_sub(1)
    }

    pub fn north_east(&self) -> Option<Coordinate> {
        self.checked_add(17)
    }

    pub fn north_west(&self) -> Option<Coordinate> {
        self.checked_add(15)
    }

    pub fn south_east(&self) -> Option<Coordinate> {
        self.checked_sub(15)
    }

    pub fn south_west(&self) -> Option<Coordinate> {
        self.checked_sub(17)
    }

    pub fn is_at_end_of_rank(&self) -> bool {
        self.0 == Coordinate::A8.0
            || self.0 == Coordinate::B8.0
            || self.0 == Coordinate::C8.0
            || self.0 == Coordinate::D8.0
            || self.0 == Coordinate::E8.0
            || self.0 == Coordinate::F8.0
            || self.0 == Coordinate::G8.0
            || self.0 == Coordinate::H8.0
    }

    fn is_on_board(&self) -> bool {
        (self.0 & 0x88) == 0
    }

    fn checked_add(&self, input: u8) -> Option<Coordinate> {
        let result = Coordinate(self.0 + input);
        if !result.is_on_board() { None }
        else { Some(result) }
    }

    fn checked_sub(&self, input: u8) -> Option<Coordinate> {
        let result = self.0.checked_sub(input)
            .map(|x| Coordinate(x));
        if let Some(coordinate) = result {
            if coordinate.is_on_board() { Some(coordinate) }
            else { None }
        } else { None }
    }

    pub fn is_last_rank(&self) -> bool {
        self.0 > 111
    }

    pub fn is_rank_7(&self) -> bool {
        (self.0 >> 4) == 6
    }

    pub fn is_rank_2(&self) -> bool {
        (self.0 >> 4) == 1
    }

    pub fn is_first_rank(&self) -> bool {
        self.0 < 8
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SAN_LOOKUP[self.0 as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_coordinate_from_usize() {
        let result = Coordinate::from_usize_no_bounds_check(0 as usize);

        assert_eq!(0 as u8, result.0)
    }

    #[test]
    fn creating_coordinate_from_usize_does_not_perform_boundary_checks() {
        let result = Coordinate::from_usize_no_bounds_check(255 as usize);

        assert_eq!(255 as u8, result.0)
    }

    #[test]
    fn create_coordinate_from_u8() {
        let result = Coordinate::from_u8_no_bounds_check(10);

        assert_eq!(10, result.0);
    }

    #[test]
    fn create_coordinate_from_u8_does_not_perform_boundary_checks() {
        let result = Coordinate::from_u8_no_bounds_check(255);

        assert_eq!(255, result.0);
    }

    #[test]
    fn convert_to_usize() {
        let coordinate = Coordinate::C2;

        let result = coordinate.as_usize();

        assert_eq!(18 as usize, result);
    }

    #[test]
    fn file_returns_str_file_for_coordinate() {
        let coordinate = Coordinate::A1;

        let result = coordinate.file();

        assert_eq!("a", result);
    }

    #[test]
    fn north() {
        let coordinate = Coordinate::E4;

        let result = coordinate.north().unwrap();

        assert_eq!(Coordinate::E5, result);
    }

    #[test]
    fn east() {
        let coordinate = Coordinate::E4;

        let result = coordinate.east().unwrap();

        assert_eq!(Coordinate::F4, result);
    }

    #[test]
    fn south() {
        let coordinate = Coordinate::E4;

        let result = coordinate.south().unwrap();

        assert_eq!(Coordinate::E3, result);
    }

    #[test]
    fn west() {
        let coordinate = Coordinate::E4;

        let result = coordinate.west().unwrap();

        assert_eq!(Coordinate::D4, result);
    }

    #[test]
    fn north_east() {
        let coordinate = Coordinate::E4;

        let result = coordinate.north_east().unwrap();

        assert_eq!(Coordinate::F5, result);
    }

    #[test]
    fn north_west() {
        let coordinate = Coordinate::E4;

        let result = coordinate.north_west().unwrap();

        assert_eq!(Coordinate::D5, result);
    }

    #[test]
    fn south_east() {
        let coordinate = Coordinate::E4;

        let result = coordinate.south_east().unwrap();

        assert_eq!(Coordinate::F3, result);
    }

    #[test]
    fn south_west() {
        let coordinate = Coordinate::E4;

        let result = coordinate.south_west().unwrap();

        assert_eq!(Coordinate::D3, result);
    }

    #[test]
    fn given_out_of_bounds_north_returns_none() {
        let coordinate = Coordinate::H8;

        let result = coordinate.north();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_east_returns_none() {
        let coordinate = Coordinate::H8;

        let result = coordinate.east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_south_returns_none() {
        let coordinate = Coordinate::A1;

        let result = coordinate.south();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_west_returns_none() {
        let coordinate = Coordinate::A1;

        let result = coordinate.west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_north_east_returns_none() {
        let coordinate = Coordinate::H8;

        let result = coordinate.north_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_north_west_returns_none() {
        let coordinate = Coordinate::H8;

        let result = coordinate.north_west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_south_east_returns_none() {
        let coordinate = Coordinate::A1;

        let result = coordinate.south_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_out_of_bounds_south_west_returns_none() {
        let coordinate = Coordinate::A1;

        let result = coordinate.south_west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_north_from_top_rank_returns_none() {
        let coordinate = Coordinate::B8;

        let result = coordinate.north();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_east_from_last_file_returns_none() {
        let coordinate = Coordinate::H5;

        let result = coordinate.east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_south_from_first_rank_returns_none() {
        let coordinate = Coordinate::B1;

        let result = coordinate.south();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_west_from_first_file_returns_none() {
        let coordinate = Coordinate::A5;

        let result = coordinate.west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_north_east_from_top_rank_returns_none() {
        let coordinate = Coordinate::A8;

        let result = coordinate.north_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_north_west_from_top_rank_returns_none() {
        let coordinate = Coordinate::B8;

        let result = coordinate.north_west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_south_east_from_first_rank_returns_none() {
        let coordinate = Coordinate::H1;

        let result = coordinate.south_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_move_south_west_from_first_rank_returns_none() {
        let coordinate = Coordinate::H1;

        let result = coordinate.south_west();

        assert_eq!(None, result);
    }
    
    #[test]
    fn given_a3_coord_south_west_returns_none() {
        let coordinate = Coordinate::A3;

        let result = coordinate.south_west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_h2_coord_south_east_returns_none() {
        let coordinate = Coordinate::H2;

        let result = coordinate.south_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_h2_coord_north_east_returns_none() {
        let coordinate = Coordinate::H2;

        let result = coordinate.north_east();

        assert_eq!(None, result);
    }

    #[test]
    fn given_a3_coord_north_west_returns_none() {
        let coordinate = Coordinate::A3;

        let result = coordinate.north_west();

        assert_eq!(None, result);
    }

    #[test]
    fn given_h7_is_not_last_rank() {
        let coordinate = Coordinate::H7;

        let result = coordinate.is_last_rank();

        assert_eq!(false, result);
    }

    #[test]
    fn given_a8_is_last_rank() {
        let coordinate = Coordinate::A8;

        let result = coordinate.is_last_rank();

        assert_eq!(true, result);
    }

    #[test]
    fn given_a2_is_not_first_rank() {
        let coordinate = Coordinate::A2;

        let result = coordinate.is_first_rank();

        assert_eq!(false, result);
    }

    #[test]
    fn given_h1_is__first_rank() {
        let coordinate = Coordinate::H1;

        let result = coordinate.is_first_rank();

        assert_eq!(true, result);
    }

    #[test]
    fn given_h6_is_not_rank_7() {
        let coordinate = Coordinate::H6;

        let result = coordinate.is_rank_7();

        assert_eq!(false, result);
    }

    #[test]
    fn given_a7_is_rank_7() {
        let coordinate = Coordinate::A7;

        let result = coordinate.is_rank_7();

        assert_eq!(true, result);
    }

    #[test]
    fn given_a3_is_not_rank_3() {
        let coordinate = Coordinate::A3;

        let result = coordinate.is_rank_2();

        assert_eq!(false, result);
    }

    #[test]
    fn given_a2_is_rank_2() {
        let coordinate = Coordinate::A2;

        let result = coordinate.is_rank_2();

        assert_eq!(true, result);
    }
    
    #[test]
    fn given_e1_is_not_at_end_of_rank() {
        let coordinate = Coordinate::E1;

        let result = coordinate.is_at_end_of_rank();

        assert_eq!(false, result);
    }

    #[test]
    fn given_e8_is_at_end_of_rank() {
        let coordinate = Coordinate::E8;

        let result = coordinate.is_at_end_of_rank();

        assert_eq!(true, result);
    }
}
