extern crate test;

use std::cmp::Eq;

pub fn difference<T: Clone + Eq>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  // deduplicate v1 and v2
  let mut a: Vec<T> = v1.clone();
  let mut b: Vec<T> = v2.clone();

  a.dedup();
  b.dedup();

  // collect difference from right to left
  a.retain(|x| !b.contains(x));

  a
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
    
    assert_eq!(difference(v1, v2), vec![1]);
  }

  #[bench]
  fn bench_difference(b: &mut Bencher) {
    // brenchmark with 10 duplicated items
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2 = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| difference(v1.clone(), v2.clone()));
  }
}
