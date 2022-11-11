use std::cmp::Ordering;
use crate::moves::Move;

#[derive(Debug)]
pub struct PossibleMove<'a> {
    pub value: i32,
    pub possible_move: &'a Move,
    pub is_maximising: bool
}

impl<'a> PartialEq<Self> for PossibleMove<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<'a> Eq for PossibleMove<'a> {}

impl<'a> PartialOrd for PossibleMove<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.value.partial_cmp(&other.value);
        if self.is_maximising { result } else { result.map(|o| o.reverse()) }
    }

    fn lt(&self, other: &Self) -> bool {
        let result = self.value.lt(&other.value);
        if self.is_maximising { result } else { !result }
    }

    fn le(&self, other: &Self) -> bool {
        let result = self.value.le(&other.value);
        if self.is_maximising { result } else { !result || other.value.eq(&self.value) }
    }

    fn gt(&self, other: &Self) -> bool {
        let result = self.value.gt(&other.value);
        if self.is_maximising { result } else { !result }
    }

    fn ge(&self, other: &Self) -> bool {
        let result = self.value.ge(&other.value);
        if self.is_maximising { result } else { !result || other.value.eq(&self.value) }
    }
}

impl<'a> Ord for PossibleMove<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
