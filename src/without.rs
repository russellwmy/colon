extern crate test;
use std::cmp::PartialEq;

pub fn without<T: Clone + PartialEq>(v: Vec<T>, vals: Vec<T>) -> Vec<T> {
  let mut o = v.clone();

  o.retain(|x| !vals.contains(x));
  o
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_v = without(v, vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(new_v, vec![1]);
  }

  #[bench]
  fn bench_without(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| without(v.clone(), vec![2, 3, 4, 5, 6, 7, 8, 9, 10]));
  }
}
