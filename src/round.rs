extern crate test;
use num_traits::{cast::FromPrimitive, float::Float, sign::Signed};

pub fn round<T: Float + Signed + FromPrimitive>(v: T, precision: i32) -> T {
  let p = T::from_isize(10isize.pow(precision.abs() as u32)).unwrap();
  match precision {
    x if x < 0 => (v / p).round() * p,
    _ => (v * p).round() / p,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(round(4.006, 0), 4.0);
    assert_eq!(round(4.006, 2), 4.01);
    assert_eq!(round(4060.0, -2), 4100.0);
  }

  #[bench]
  fn bench_round(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| round(4060.0, -2));
  }
}
