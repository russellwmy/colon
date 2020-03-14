extern crate test;

pub fn fill<T: Clone>(v: Vec<T>, val: T) -> Vec<T> {
  v.into_iter().map(|_| val.clone()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      fill::<i32>([1; 10].to_vec(), 2),
      [2; 10]
    );
  }

  #[bench]
  fn bench_fill(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| fill::<i32>([1; 10].to_vec(), 2));
  }
}
