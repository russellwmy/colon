extern crate test;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;

pub fn intersection<T: Clone + Hash + PartialEq + Eq + Ord>(
  mut v1: Vec<T>,
  mut v2: Vec<T>,
) -> Vec<T> {
  // deduplicate v1 and v2
  let a: HashSet<T> = v1.drain(..).collect();
  let b: HashSet<T> = v2.drain(..).collect();
  

  a.intersection(&b).map(|x| x.to_owned()).collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 duplicated items
      intersection::<i32>([1; 10].to_vec(), vec![1, 2, 3]).len(),
      1
    );
  }

  #[bench]
  fn bench_intersection(b: &mut Bencher) {
    // brenchmark with 10 duplicated items
    b.iter(|| intersection::<i32>([1; 10].to_vec(), vec![1, 2, 3]));
  }
}
