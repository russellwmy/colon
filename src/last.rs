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
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(last(v), Some(10));
  }

  #[bench]
  fn bench_last(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| last(v.clone()));
  }
}
