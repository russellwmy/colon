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
    // test with 10 items
    let v: Vec<i32> = [1; 10].to_vec();

    assert_eq!(fill(v, 2), [2; 10]);
  }

  #[bench]
  fn bench_fill(b: &mut Bencher) {
    let v: Vec<i32> = [1; 10].to_vec();
    // benchmark with 10 items
    b.iter(|| fill(v.clone(), 2));
  }
}
