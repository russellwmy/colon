extern crate test;
use std::ops::Fn;

pub fn remove<T: Clone, F>(v: &mut Vec<T>, f: F) -> Vec<T>
where
  F: Fn(T) -> bool,
{
  let mut removed = v.clone();
  
  v.retain(|x| !f(x.to_owned()));
  removed.retain(|x| f(x.to_owned()));

  removed
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let f = |val| val % 2 == 0;
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(remove(&mut v, f), vec![2, 4, 6, 8, 10]);
  }

  #[bench]
  fn bench_remove(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| {
      let mut v = [2;1000].to_vec();
      let f = |val| val % 2 == 0;

      remove(&mut v, f)
    });
  }
}
