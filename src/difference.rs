extern crate test;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn difference<T: Clone + Hash + PartialEq + Eq + Ord>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  // deduplicate v1 and v2
  let a: HashSet<T> = HashSet::from_iter(v1.iter().cloned());
  let b: HashSet<T> = HashSet::from_iter(v2.iter().cloned());
  // collect difference from right to left
  let mut l = a.difference(&b).map(|x| x.to_owned()).collect::<Vec<T>>();
  l
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 duplicated items
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2 = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(difference::<i32>(v1, v2), vec![1]);
  }

  #[bench]
  fn bench_difference(b: &mut Bencher) {
    // brenchmark with 10 duplicated items
    b.iter(|| {
      let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      let v2 = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
      difference::<i32>(v1, v2)
    });
  }
}
