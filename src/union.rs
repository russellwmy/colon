extern crate test;

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn union<T: Clone + Hash + PartialEq + Eq>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  let a: HashSet<T> = HashSet::from_iter(v1.iter().cloned());
  let b: HashSet<T> = HashSet::from_iter(v2.iter().cloned());
  a.union(&b).map(|x| x.to_owned()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    assert_eq!(union(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]).len(), 10);
  }

  #[bench]
  fn bench_union(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| union(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]));
  }
}
