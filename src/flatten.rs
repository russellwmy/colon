extern crate test;

pub fn flatten<T>(v: Vec<Vec<T>>) -> Vec<T> {
  v.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];

    assert_eq!(flatten(v.clone()).len(), 10);
  }

  #[bench]
  fn bench_flatten(b: &mut Bencher) {
    // benchmark with 10 items
    let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];

    b.iter(|| flatten(v.clone()));
  }
}
