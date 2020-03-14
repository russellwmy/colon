extern crate test;

use num_traits::{cast::FromPrimitive, float::Float, sign::Signed};

pub fn floor<T: Float + Signed + FromPrimitive>(v: T, precision: i32) -> T {
  let p = T::from_isize(10isize.pow(precision.abs() as u32)).unwrap();

  match precision {
    x if x < 0 => (v / p).floor() * p,
    _ => (v * p).floor() / p,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(floor(4.006, 0), 4.0);
    assert_eq!(floor(0.046, 2), 0.04);
    assert_eq!(floor(4060.0, -2), 4000.0);
  }

  #[bench]
  fn bench_floor(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| floor(4060.0, -2));
  }
}
