extern crate test;

use std::cmp::Eq;

pub fn xor<T: Clone + Eq>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  // deduplicate v1 and v2
  let mut a: Vec<T> = v1.clone();
  let mut b: Vec<T> = v2.clone();

  a.dedup();
  b.dedup();

  let mut c = a.clone();
  c.extend(b.clone());

  // collect difference from right to left
  a.retain(|x| b.contains(x));
  c.retain(|x| !a.contains(x));
  c
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 duplicated items
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(xor::<i32>(v1, v2), vec![1, 10]);
  }

  #[bench]
  fn bench_xor(b: &mut Bencher) {
    // brenchmark with 10 duplicated items
    b.iter(|| {
      let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
      let v2 = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
      xor::<i32>(v1, v2)
    });
  }
}
