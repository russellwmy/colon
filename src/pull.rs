extern crate test;
use std::cmp::PartialEq;

pub fn pull<T: PartialEq>(v: &mut Vec<T>, vals: Vec<T>) {
  v.retain(|x| !vals.contains(x));
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    pull(&mut v, vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(v, vec![1]);
  }

  #[bench]
  fn bench_pull(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| {
      let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      pull(&mut v, vec![2, 3, 4, 5, 6, 7, 8, 9, 10])
    });
  }
}
