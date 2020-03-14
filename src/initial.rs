extern crate test;

pub fn initial<T: Clone>(mut v: Vec<T>) -> Vec<T> {
  v.remove(v.len() - 1);
  v
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      initial(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
  }

  #[bench]
  fn bench_initial(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| initial(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
  }
}
