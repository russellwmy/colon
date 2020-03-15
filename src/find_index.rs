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
    // test with 10 items
    let f = |val| val == 2;
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(find_index(v, f), 1);
  }

  #[bench]
  fn bench_find_index(b: &mut Bencher) {
    // benchmark with 10 items
    let f = |val| val == 2;
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    b.iter(|| find_index(v.clone(), f));
  }
}
