use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use derive_more::{Deref, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Display)]
pub struct File {
    inner: usize
}

impl File {
    pub fn new(inner: usize) -> File {
        File{inner}
    }

    pub fn transform(&self, rhs: isize) -> Option<File> {
        let result = (self.inner as isize) + rhs;
        if result < 0 { None }
        else if result > 7 { None }
        else { Some(File { inner: result as usize }) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Display)]
pub struct Rank {
    inner: usize
}

impl Rank {
    pub fn new(inner: usize) -> Rank {
        Rank{inner}
    }

    pub fn transform(&self, rhs: isize) -> Option<Rank> {
        let result = (self.inner as isize) + rhs;
        if result < 0 { None }
        else if result > 7 { None }
        else { Some(Rank { inner: result as usize }) }
    }
}

impl Add<usize> for File {
    type Output = File;

    fn add(self, rhs: usize) -> Self::Output {
        File::new(self.inner + rhs)
    }
}

impl Add<File> for usize {
    type Output = File;

    fn add(self, rhs: File) -> Self::Output {
        File::new(self + rhs.inner)
    }
}

impl Sub<usize> for File {
    type Output = File;

    fn sub(self, rhs: usize) -> Self::Output {
        File::new(self.inner - rhs)
    }
}

impl Sub<File> for usize {
    type Output = File;

    fn sub(self, rhs: File) -> Self::Output {
        File::new(self - rhs.inner)
    }
}

impl Mul<usize> for File {
    type Output = File;

    fn mul(self, rhs: usize) -> Self::Output {
        File::new(self.inner * rhs)
    }
}

impl Mul<File> for usize {
    type Output = File;

    fn mul(self, rhs: File) -> Self::Output {
        File::new(self * rhs.inner)
    }
}

impl PartialEq<usize> for File {
    fn eq(&self, other: &usize) -> bool {
        &self.inner == other
    }

    fn ne(&self, other: &usize) -> bool {
        &self.inner != other
    }
}

impl PartialEq<File> for usize {
    fn eq(&self, other: &File) -> bool {
        self == &other.inner
    }

    fn ne(&self, other: &File) -> bool {
        self != &other.inner
    }
}

impl SubAssign<usize> for File {
    fn sub_assign(&mut self, rhs: usize) {
        self.inner -= rhs;
    }
}

impl AddAssign<usize> for File {
    fn add_assign(&mut self, rhs: usize) {
        self.inner += rhs;
    }
}

impl From<File> for char {
    fn from(file: File) -> Self {
        file.inner as u8 as char
    }
}

impl Add<usize> for Rank {
    type Output = Rank;

    fn add(self, rhs: usize) -> Self::Output {
        Rank::new(self.inner + rhs)
    }
}

impl Add<Rank> for usize {
    type Output = Rank;

    fn add(self, rhs: Rank) -> Self::Output {
        Rank::new(self + rhs.inner)
    }
}

impl Sub<usize> for Rank {
    type Output = Rank;

    fn sub(self, rhs: usize) -> Self::Output {
        Rank::new(self.inner - rhs)
    }
}

impl Sub<Rank> for usize {
    type Output = Rank;

    fn sub(self, rhs: Rank) -> Self::Output {
        Rank::new(self - rhs.inner)
    }
}

impl Mul<usize> for Rank {
    type Output = Rank;

    fn mul(self, rhs: usize) -> Self::Output {
        Rank::new(self.inner * rhs)
    }
}

impl Mul<Rank> for usize {
    type Output = Rank;

    fn mul(self, rhs: Rank) -> Self::Output {
        Rank::new(self * rhs.inner)
    }
}

impl PartialEq<usize> for Rank {
    fn eq(&self, other: &usize) -> bool {
        &self.inner == other
    }

    fn ne(&self, other: &usize) -> bool {
        &self.inner != other
    }
}

impl PartialEq<Rank> for usize {
    fn eq(&self, other: &Rank) -> bool {
        self == &other.inner
    }

    fn ne(&self, other: &Rank) -> bool {
        self != &other.inner
    }
}

impl SubAssign<usize> for Rank {
    fn sub_assign(&mut self, rhs: usize) {
        self.inner -= rhs;
    }
}

impl AddAssign<usize> for Rank {
    fn add_assign(&mut self, rhs: usize) {
        self.inner += rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_should_equal_integer_regardless_of_order() {
        let file = File::new(8);

        assert_eq!(8, file);
        assert_eq!(file, 8);
    }

    #[test]
    fn file_should_add_integer_regardless_of_order() {
        let result = File::new(5) + 3;
        let result_reverse = 3 + File::new(5);

        assert_eq!(8, result);
        assert_eq!(8, result_reverse);
    }

    #[test]
    fn file_should_subtract_integer_regardless_of_order() {
        let result = File::new(5) - 3;
        let result_reverse = 5 - File::new(3);

        assert_eq!(2, result);
        assert_eq!(2, result_reverse);
    }

    #[test]
    fn file_should_multiply_integer_regardless_of_order() {
        let result = File::new(2) * 4;
        let result_reverse = 4 * File::new(2);

        assert_eq!(8, result);
        assert_eq!(8, result_reverse);
    }

    #[test]
    fn file_should_subtract_assign_integer() {
        let mut result = File::new(2);
        result -= 1;

        assert_eq!(1, result);
    }

    #[test]
    fn file_should_add_assign_integer() {
        let mut result = File::new(2);
        result += 1;

        assert_eq!(3, result);
    }

    #[test]
    fn file_should_transform_negative_number() {
        let result = File::new(7).transform(-2).unwrap();

        assert_eq!(5, result);
    }

    #[test]
    fn file_transform_should_return_none_if_result_negative() {
        let result = File::new(1).transform(-2);

        assert!(result.is_none());
    }

    #[test]
    fn file_transform_should_return_none_if_result_too_big() {
        let result = File::new(7).transform(1);

        assert!(result.is_none());
    }

    #[test]
    fn rank_should_equal_integer_regardless_of_order() {
        let rank = Rank::new(8);

        assert_eq!(8, rank);
        assert_eq!(rank, 8);
    }

    #[test]
    fn rank_should_add_integer_regardless_of_order() {
        let result = Rank::new(5) + 3;
        let result_reverse = 3 + Rank::new(5);

        assert_eq!(8, result);
        assert_eq!(8, result_reverse);
    }

    #[test]
    fn rank_should_subtract_integer_regardless_of_order() {
        let result = Rank::new(5) - 3;
        let result_reverse = 5 - Rank::new(3);

        assert_eq!(2, result);
        assert_eq!(2, result_reverse);
    }

    #[test]
    fn rank_should_multiply_integer_regardless_of_order() {
        let result = Rank::new(2) * 4;
        let result_reverse = 4 * Rank::new(2);

        assert_eq!(8, result);
        assert_eq!(8, result_reverse);
    }

    #[test]
    fn rank_should_subassign_integer() {
        let mut result = Rank::new(2);
        result -= 1;

        assert_eq!(1, result);
    }

    #[test]
    fn rank_should_add_assign_integer() {
        let mut result = Rank::new(2);
        result += 1;

        assert_eq!(3, result);
    }

    #[test]
    fn rank_should_transform_negative_number() {
        let result = Rank::new(7).transform(-2).unwrap();

        assert_eq!(5, result);
    }

    #[test]
    fn rank_transform_should_return_none_if_result_negative() {
        let result = Rank::new(1).transform(-2);

        assert!(result.is_none());
    }

    #[test]
    fn rank_transform_should_return_none_if_result_too_big() {
        let result = Rank::new(7).transform(1);

        assert!(result.is_none());
    }
}
