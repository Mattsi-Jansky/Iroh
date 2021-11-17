#[derive(PartialEq)]
#[derive(Debug)]
pub enum Move {
    RegularMove((usize, usize), (usize, usize)),
    PawnMove(usize,(usize,usize))
}
