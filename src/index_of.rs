use crate::find_index;

use std::cmp::PartialEq;

pub fn index_of<T: Clone + PartialEq>(v: Vec<T>, val: T) -> usize {
  find_index::find_index(v, |x| x == val)
}
