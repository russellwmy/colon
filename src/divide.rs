extern crate test;

use num_traits::{cast::FromPrimitive, sign::Signed, float::Float};

pub fn divide<T: Float + Signed + FromPrimitive>(dividend: T, divisor: T) -> T {
  dividend / divisor
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(divide(6.0, 4.0), 1.5);
  }

  #[bench]
  fn bench_divide(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| divide(6.0, 4.0));
  }
}
