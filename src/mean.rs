extern crate test;

use std::cmp::PartialEq;
use std::iter::Sum;
use std::ops::Div;

use num_traits::cast::{FromPrimitive, ToPrimitive};

pub fn mean<T: Copy + Sum + Div + PartialEq + FromPrimitive + ToPrimitive>(v: Vec<T>) -> f64 {
  let s = v.clone().into_iter().sum::<T>();
  let r = s.to_f64().unwrap() / v.len() as f64;

  r
}
#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(mean(v), 5.5);
  }

  #[bench]
  fn bench_mean(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| mean(v.clone()));
  }
}
