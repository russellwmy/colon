extern crate test;

pub fn take_right<T: Clone>(v: Vec<T>, n: usize) -> Vec<T> {
  let l = v.len();
  v[l - n..l].into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      take_right(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4),
      vec![7, 8, 9, 10]
    );
  }

  #[bench]
  fn bench_take_right(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| take_right(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4));
  }
}
