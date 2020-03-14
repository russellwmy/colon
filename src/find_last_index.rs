extern crate test;

use std::cmp::Ord;
use std::ops::Fn;

pub fn find_last_index<T: Clone + Ord, F>(mut v: Vec<T>, f: F) -> usize
where
  F: Fn(T) -> bool,
{
  v.reverse();
  v.into_iter().position(|e| f(e)).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let f = |val| val == 9;
    assert_eq!(
      // test with 10 items
      find_last_index([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), f),
      1
    );
  }

  #[bench]
  fn bench_find_last_index(b: &mut Bencher) {
    // benchmark with 10 items
    let f = |val| val == 9;
    b.iter(|| find_last_index([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), f));
  }
}
