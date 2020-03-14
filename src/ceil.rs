extern crate test;
use num_traits::{cast::FromPrimitive, float::Float, sign::Signed};

pub fn ceil<T: Float + Signed + FromPrimitive>(v: T, precision: i32) -> T {
  let p = T::from_isize(10isize.pow(precision.abs() as u32)).unwrap();
  match precision {
    x if x < 0 => (v / p).ceil() * p,
    _ => (v * p).ceil() / p,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(ceil(4.006, 0), 5.0);
    assert_eq!(ceil(6.004, 2), 6.01);
    assert_eq!(ceil(6040.0, -2), 6100.0);
  }

  #[bench]
  fn bench_ceil(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| ceil(6040.0, -2));
  }
}
