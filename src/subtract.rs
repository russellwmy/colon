extern crate test;

use std::ops::Sub;

use num_traits::cast::ToPrimitive;

pub fn subtract<A, B>(v1: A, v2: B) -> f64
where
  A: Copy + Sub + ToPrimitive,
  B: Copy + Sub + ToPrimitive,
{
  v1.to_f64().unwrap() - v2.to_f64().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(subtract(3, 1), 2.0);
  }

  #[bench]
  fn bench_subtract(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| subtract(3, 1));
  }
}
