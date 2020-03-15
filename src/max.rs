extern crate test;

use std::cmp::Ord;

pub fn max<T: Copy + Ord>(v: Vec<T>) -> T {
  *v.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(max(v), 10);
  }

  #[bench]
  fn bench_max(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| max(v.clone()));
  }
}
