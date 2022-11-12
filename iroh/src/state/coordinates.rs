use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use derive_more::{Deref, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Hash)]
pub struct Coordinate(u8);

const SAN_LOOKUP: [&str; 8*8] = [
    "a1","a2","a3","a4","a5","a6","a7","a8",
    "b1","b2","b3","b4","b5","b6","b7","b8",
    "c1","c2","c3","c4","c5","c6","c7","c8",
    "d1","d2","d3","d4","d5","d6","d7","d8",
    "e1","e2","e3","e4","e5","e6","e7","e8",
    "f1","f2","f3","f4","f5","f6","f7","f8",
    "g1","g2","g3","g4","g5","g6","g7","g8",
    "h1","h2","h3","h4","h5","h6","h7","h8",
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

    pub const A2: Coordinate = Coordinate(8);
    pub const B2: Coordinate = Coordinate(9);
    pub const C2: Coordinate = Coordinate(10);
    pub const D2: Coordinate = Coordinate(11);
    pub const E2: Coordinate = Coordinate(12);
    pub const F2: Coordinate = Coordinate(13);
    pub const G2: Coordinate = Coordinate(14);
    pub const H2: Coordinate = Coordinate(15);

    pub const A3: Coordinate = Coordinate(16);
    pub const B3: Coordinate = Coordinate(17);
    pub const C3: Coordinate = Coordinate(18);
    pub const D3: Coordinate = Coordinate(19);
    pub const E3: Coordinate = Coordinate(20);
    pub const F3: Coordinate = Coordinate(21);
    pub const G3: Coordinate = Coordinate(22);
    pub const H3: Coordinate = Coordinate(23);

    pub const A4: Coordinate = Coordinate(24);
    pub const B4: Coordinate = Coordinate(25);
    pub const C4: Coordinate = Coordinate(26);
    pub const D4: Coordinate = Coordinate(27);
    pub const E4: Coordinate = Coordinate(28);
    pub const F4: Coordinate = Coordinate(29);
    pub const G4: Coordinate = Coordinate(30);
    pub const H4: Coordinate = Coordinate(31);

    pub const A5: Coordinate = Coordinate(32);
    pub const B5: Coordinate = Coordinate(33);
    pub const C5: Coordinate = Coordinate(34);
    pub const D5: Coordinate = Coordinate(35);
    pub const E5: Coordinate = Coordinate(36);
    pub const F5: Coordinate = Coordinate(37);
    pub const G5: Coordinate = Coordinate(38);
    pub const H5: Coordinate = Coordinate(39);

    pub const A6: Coordinate = Coordinate(40);
    pub const B6: Coordinate = Coordinate(41);
    pub const C6: Coordinate = Coordinate(42);
    pub const D6: Coordinate = Coordinate(43);
    pub const E6: Coordinate = Coordinate(44);
    pub const F6: Coordinate = Coordinate(45);
    pub const G6: Coordinate = Coordinate(46);
    pub const H6: Coordinate = Coordinate(47);

    pub const A7: Coordinate = Coordinate(48);
    pub const B7: Coordinate = Coordinate(49);
    pub const C7: Coordinate = Coordinate(50);
    pub const D7: Coordinate = Coordinate(51);
    pub const E7: Coordinate = Coordinate(52);
    pub const F7: Coordinate = Coordinate(53);
    pub const G7: Coordinate = Coordinate(54);
    pub const H7: Coordinate = Coordinate(55);

    pub const A8: Coordinate = Coordinate(56);
    pub const B8: Coordinate = Coordinate(57);
    pub const C8: Coordinate = Coordinate(58);
    pub const D8: Coordinate = Coordinate(59);
    pub const E8: Coordinate = Coordinate(61);
    pub const F8: Coordinate = Coordinate(62);
    pub const G8: Coordinate = Coordinate(63);
    pub const H8: Coordinate = Coordinate(64);

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
        //wrap check
        if self.0 > 55 {
            None
        } else {
            self.checked_add(8)
        }
    }

    pub fn east(&self) -> Option<Coordinate> {
        //wrap check
        if self.is_at_end_of_rank() {
            None
        }
        else { self.checked_add(1) }
    }

    pub fn south(&self) -> Option<Coordinate> {
        if self.0 < 8 { //wrap check
            None
        } else { self.0.checked_sub(8).map(|x| Coordinate(x)) }
    }

    pub fn west(&self) -> Option<Coordinate> {
        //wrawp check
        if self.is_at_start_of_rank() {
            None
        } else {
            self.0.checked_sub(1).map(|x| Coordinate(x))
        }
    }

    pub fn north_east(&self) -> Option<Coordinate> {
        self.checked_add(9)
    }

    pub fn north_west(&self) -> Option<Coordinate> {
        if self.0 > 55 { //wrap check
            None
        } else {
            self.checked_add(7)
        }
    }

    pub fn south_east(&self) -> Option<Coordinate> {
        if self.0 < 8 { //wrap check
            None
        } else {
            self.0.checked_sub(7).map(|x| Coordinate(x))
        }
    }

    pub fn is_at_end_of_rank(&self) -> bool {
        self.0 == 63 || self.0 == 55 || self.0 == 47 || self.0 == 39 || self.0 == 31 || self.0 == 23 || self.0 == 15 || self.0 == 7
    }

    pub fn is_at_start_of_rank(&self) -> bool {
        self.0 == 0 || self.0 == 56 || self.0 == 48 || self.0 == 40 || self.0 == 32 || self.0 == 24 || self.0 == 16 || self.0 == 8
    }

    pub fn south_west(&self) -> Option<Coordinate> {
        self.0.checked_sub(9).map(|x| Coordinate(x))
    }

    fn checked_add(&self, input: u8) -> Option<Coordinate> {
        let result = self.0 + input;
        if result > 64 { None }
        else { Some(Coordinate(result)) }
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

        assert_eq!(10 as usize, result);
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
}
