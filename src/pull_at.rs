extern crate test;
use std::cmp::PartialEq;

pub fn pull_at<T: PartialEq>(v: &mut Vec<T>, mut idxs: Vec<usize>) -> Vec<T> {
  idxs.sort();
  idxs.reverse();
  idxs.iter().map(|x| v.remove(*x)).collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let pulled = pull_at(&mut v, vec![0, 9, 1]);

    assert_eq!(v, vec![3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(pulled, vec![10, 2, 1]);
  }

  #[bench]
  fn bench_pull_at(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| {
      let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      pull_at(&mut v, vec![0, 9, 1])
    });
  }
}
