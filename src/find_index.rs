extern crate test;
use std::ops::Fn;

pub fn find_index<T: Clone, F>(v: Vec<T>, f: F) -> usize
where
  F: Fn(T) -> bool,
{
  v.into_iter().position(|e| f(e)).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let f = |val| val == 2;
    assert_eq!(
      // test with 10 items
      find_index([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), f),
      1
    );
  }

  #[bench]
  fn bench_find_index(b: &mut Bencher) {
    // benchmark with 10 items
    let f = |val| val == 2;
    b.iter(|| find_index([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), f));
  }
}
