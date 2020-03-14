extern crate test;

use std::iter::Sum;

use num_traits::cast::ToPrimitive;

pub fn sum<T: Copy + Sum + ToPrimitive>(v: Vec<T>) -> f64 {
  let s = v.clone().into_iter().sum::<T>();
  s.to_f64().unwrap()
}
#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55.0);
  }

  #[bench]
  fn bench_sum(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
  }
}
