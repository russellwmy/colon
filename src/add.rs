extern crate test;
use num_traits::{cast::FromPrimitive, sign::Signed};

pub fn add<T: Signed + FromPrimitive>(v1: T, v2: T) -> T {
  v1 + v2
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(add(5.0, 5.0), 10.0);
  }

  #[bench]
  fn bench_add(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| add(5.0, 5.0));
  }
}
