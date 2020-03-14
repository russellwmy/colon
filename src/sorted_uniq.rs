extern crate test;

use std::cmp::PartialEq;
use std::cmp::PartialOrd;

pub fn sorted_uniq<T: Clone + PartialEq + PartialOrd>(v: Vec<T>) -> Vec<T> {
  let mut o = v.clone();
  o.dedup();
  o
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    assert_eq!(
      sorted_uniq(vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10]),
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
  }

  #[bench]
  fn bench_sorted_uniq(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| sorted_uniq(vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10]));
  }
}
