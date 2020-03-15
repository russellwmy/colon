extern crate test;

pub fn slice<T: Clone>(v: Vec<T>, start: usize, end: usize) -> Vec<T> {
  v[start..end].into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(slice(v, 1, 4), vec![2, 3, 4]);
  }

  #[bench]
  fn bench_slice(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| slice(v.clone(), 1, 4));
  }
}
