extern crate test;

use std::ops::Mul;

use num_traits::cast::ToPrimitive;

pub fn multiply<A, B>(v1: A, v2: B) -> f64
where
  A: Copy + Mul + ToPrimitive,
  B: Copy + Mul + ToPrimitive,
{
  v1.to_f64().unwrap() * v2.to_f64().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(multiply(1, 3.0), 3.0);
  }

  #[bench]
  fn bench_multiply(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| multiply(1, 3.0));
  }
}
