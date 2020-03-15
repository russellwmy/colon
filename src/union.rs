extern crate test;

use std::cmp::PartialEq;

pub fn union<T: Clone + PartialEq>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  let mut o = v1.clone();

  o.extend(v2);
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
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9, 10];

    assert_eq!(union(v1, v2).len(), 10);
  }

  #[bench]
  fn bench_union(b: &mut Bencher) {
    // benchmark with 10 items
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9, 10];
    b.iter(|| union(v1.clone(), v2.clone()));
  }
}
