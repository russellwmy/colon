use crate::find_last_index;

use std::cmp::PartialEq;
use std::cmp::Ord;

pub fn last_index_of<T: Clone + PartialEq + Ord>(v: Vec<T>, val: T) -> usize {
  find_last_index::find_last_index(v, |x| x == val)
}
