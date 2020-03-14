extern crate test;

pub fn take<T: Clone>(v: Vec<T>, n: usize) -> Vec<T> {
  v[..n].into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      take(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4),
      vec![1, 2, 3, 4]
    );
  }

  #[bench]
  fn bench_take(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| take(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4));
  }
}
