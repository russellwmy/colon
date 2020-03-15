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
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(min(v), 1);
  }

  #[bench]
  fn bench_min(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| min(v.clone()));
  }
}
