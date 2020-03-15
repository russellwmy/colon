extern crate test;

use crate::zip;

pub fn unzip<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  zip::zip(v)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
    assert_eq!(unzip(v), vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]]);
  }

  #[bench]
  fn bench_unzip(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];

    b.iter(|| unzip(v.clone()));
  }
}