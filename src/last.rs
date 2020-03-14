extern crate test;

pub fn last<T: Clone>(v: Vec<T>) -> Option<T> {
  v.last().cloned()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      last([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec()),
      Some(10)
    );
  }

  #[bench]
  fn bench_last(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| last([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec()));
  }
}
