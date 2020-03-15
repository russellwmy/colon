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
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(sum(v), 55.0);
  }

  #[bench]
  fn bench_sum(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| sum(v.clone()));
  }
}
