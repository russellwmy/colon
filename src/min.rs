extern crate test;

use std::cmp::Ord;

pub fn min<T: Copy + Ord>(v: Vec<T>) -> T {
  *v.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(min(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
  }

  #[bench]
  fn bench_min(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| min(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
  }
}
